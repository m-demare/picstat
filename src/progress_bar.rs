use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::CliArgs;

pub fn build_progress_bar(args: &CliArgs, total_count: usize) -> ProgressBar {
    let pb_chars = if args.hist_char == '█' {
        "█▉▊▋▌▍▎▏  "
    } else {
        "#>-"
    };

    ProgressBar::new(
        total_count
            .try_into()
            .expect("Probably shouldn't have that many files :p"),
    )
    .with_style(
        ProgressStyle::with_template(
            " [{elapsed_precise}] |{wide_bar:.cyan/blue}| {pos:>7}/{len:7} ETA: {eta} {msg} ",
        )
        .expect("Template is valid")
        .progress_chars(pb_chars),
    )
}
