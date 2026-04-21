#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArxivPaper {
    pub id: &'static str,
    pub title: &'static str,
}

pub fn ingest_with_seaorm_stub() -> Vec<ArxivPaper> {
    vec![ArxivPaper {
        id: "arXiv:1706.03762",
        title: "Attention Is All You Need",
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_seed_paper() {
        let papers = ingest_with_seaorm_stub();
        assert_eq!(papers.len(), 1);
        assert_eq!(papers[0].id, "arXiv:1706.03762");
    }
}
