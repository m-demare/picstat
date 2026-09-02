use crate::Context;
use crate::cli::CliArgs;
use crate::file_process::{ParsedMetadata, parse_metadata, process_metadata};
use rayon::iter::ParallelIterator;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator};
use std::path::Path;
use walkdir::WalkDir;

pub fn process_dir(dir: &Path, args: &CliArgs, ctxt: &mut Context) -> std::io::Result<()> {
    let total_count = count_files(dir, args);

    ctxt.initialize_progress_bar(args, total_count);

    let walker = build_walker(dir, args);

    walk(walker, args, ctxt)?;

    ctxt.finish_analysis(args);

    Ok(())
}

fn walk<I: Iterator<Item = Result<walkdir::DirEntry, walkdir::Error>>>(
    mut walker: I,
    args: &CliArgs,
    ctxt: &mut Context,
) -> std::io::Result<()> {
    const BATCH_SIZE: usize = 2 << 10;
    let mut batch = Vec::with_capacity(BATCH_SIZE);
    let mut metadata_buffer = Vec::with_capacity(BATCH_SIZE);
    loop {
        batch.clear();
        for entry in (&mut walker).take(BATCH_SIZE) {
            batch.push(entry?);
        }

        if batch.is_empty() {
            return Ok(());
        }

        batch
            .par_iter()
            .map(|f| {
                if f.file_type().is_file() {
                    ctxt.analyse_file();
                    parse_metadata(f.path(), args, ctxt)
                } else if f.file_type().is_dir() {
                    ctxt.analyse_dir();
                    Ok(ParsedMetadata::Silent)
                } else {
                    Ok(ParsedMetadata::Silent)
                }
            })
            .collect_into_vec(&mut metadata_buffer);

        #[expect(clippy::iter_with_drain)]
        for metadata in metadata_buffer.drain(..) {
            process_metadata(ctxt, metadata?);
        }
    }
}

fn count_files(dir: &Path, args: &CliArgs) -> usize {
    let walker = build_walker(dir, args);
    walker
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.file_type().is_file())
        .count()
}

fn build_walker(
    dir: &Path,
    args: &CliArgs,
) -> impl Iterator<Item = Result<walkdir::DirEntry, walkdir::Error>> {
    let mut walker = WalkDir::new(dir);

    if !args.recursive {
        walker = walker.max_depth(1);
    }

    walker
        .into_iter()
        .filter_entry(|f| !f.file_type().is_file() || args.should_analyse(f.path()))
}
