// SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
// SPDX-License-Identifier: 0BSD

use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{Result, Write};

use code_generator::catalog;

fn main() -> Result<()> {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let dep5_file = format!("{}/.reuse/dep5", root);
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(dep5_file)
        .expect("unable to open file");
    let mut buffer = BufWriter::new(file);

    writeln!(
        buffer,
        "Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/"
    )?;
    writeln!(buffer, "Upstream-Name: kube-custom-resources-rs")?;
    writeln!(buffer, "Upstream-Contact: Sebastian Hoß <seb@hoß.de>")?;
    writeln!(
        buffer,
        "Source: https://github.com/metio/kube-custom-resources-rs"
    )?;
    writeln!(buffer, "")?;
    writeln!(buffer, "Files: kube-custom-resources-rs/src/*")?;
    writeln!(buffer, "Copyright: The kube-custom-resources-rs Authors")?;
    writeln!(buffer, "License: 0BSD")?;

    for source in catalog::CRD_V1_SOURCES {
        writeln!(buffer, "")?;
        writeln!(buffer, "Files: crd-catalog/{}/*", source.project_name)?;
        writeln!(buffer, "Copyright: The {} Authors", source.project_name)?;
        writeln!(buffer, "License: {}", source.license)?;
    }

    Ok(())
}
