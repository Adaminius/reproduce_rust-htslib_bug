use rust_htslib::bcf;
use rust_htslib::bcf::Read;

fn main() {
    let mut vcf = bcf::Reader::from_path("src/test_data/tmp.vcf")
        .expect(&format!("Failed to open file"));
    let num_samples = vcf.header().sample_count() as usize;

    for (rec_index, rec) in vcf.records().enumerate() {
        let mut record = rec.unwrap();
        let tmp = record.format(b"FT").string().unwrap();
        let genotypes = record.genotypes().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn test_read_vcf() {
        main();
    }
}
