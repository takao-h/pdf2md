// tests/mod.rs

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::path::Path;
    use tempfile::NamedTempFile;

    // 単体テスト: Markdown変換ロジック
    #[test]
    fn test_convert_to_markdown() {
        let test_cases = vec![
            (
                "Hello World",
                "Hello World",
                "通常テキストの変換"
            ),
            (
                "1. INTRODUCTION\nThis is a sample text",
                "# INTRODUCTION\n\nThis is a sample text",
                "見出しの変換"
            ),
            (
                "   THIS IS IMPORTANT   ",
                "**THIS IS IMPORTANT**",
                "強調テキストの変換"
            ),
            (
                "First paragraph\n\nSecond paragraph",
                "First paragraph\n\nSecond paragraph",
                "段落の分割"
            ),
        ];

        for (input, expected, desc) in test_cases {
            let result = convert_to_markdown(input.to_string()).unwrap();
            assert_eq!(result, expected, "Test failed: {}", desc);
        }
    }

    // 単体テスト: ファイル書き込み
    #[test]
    fn test_write_to_file() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_path_buf();
        let content = "test content";

        write_to_file(&path, content)?;
        let saved_content = std::fs::read_to_string(path)?;

        assert_eq!(content, saved_content);
        Ok(())
    }

    // 統合テスト用ヘルパー関数
    fn run_cli_test(input_path: &str, output_path: &str) -> Result<()> {
        use assert_cmd::Command;
        use predicates::prelude::*;

        let mut cmd = Command::cargo_bin("your_cli_name")?;
        cmd.arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .assert()
            .success()
            .stdout(predicate::str::contains("変換が完了しました"));

        let output_content = std::fs::read_to_string(output_path)?;
        assert!(!output_content.is_empty());
        Ok(())
    }

    // 統合テスト（実際のPDFファイルを使用）
    #[test]
    #[ignore = "実際のPDFファイルが必要なためCIでは無効化"]
    fn test_full_conversion_process() -> Result<()> {
        let output_path = "test_output.md";
        run_cli_test("tests/fixtures/sample.pdf", output_path)?;
        std::fs::remove_file(output_path)?;
        Ok(())
    }

    // エラーハンドリングテスト
    #[test]
    fn test_invalid_pdf_handling() {
        use assert_cmd::Command;
        
        let mut cmd = Command::cargo_bin("your_cli_name").unwrap();
        let output = cmd.arg("--input")
            .arg("non_existent.pdf")
            .assert()
            .failure();

        let output_str = String::from_utf8_lossy(&output.get_output().stderr);
        assert!(output_str.contains("PDFからのテキスト抽出に失敗しました"));
    }
}
