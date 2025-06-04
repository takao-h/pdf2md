use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// PDF を Markdown に変換するCLIツール
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 入力PDFファイルのパス
    #[arg(short, long)]
    input: PathBuf,

    /// 出力Markdownファイルのパス（指定がない場合は入力ファイル名に .md を付けたものになります）
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    // コマンドライン引数の解析
    let args = Args::parse();

    // 出力ファイルパスの決定
    let output_path = match args.output {
        Some(path) => path,
        None => {
            let mut path = args.input.clone();
            path.set_extension("md");
            path
        }
    };

    // PDF の内容を抽出
    let pdf_content = extract_pdf_content(&args.input)?;

    // Markdown への変換
    let markdown_content = convert_to_markdown(pdf_content)?;

    // ファイルへの書き込み
    write_to_file(&output_path, &markdown_content)?;

    println!("変換が完了しました。出力ファイル: {:?}", output_path);
    Ok(())
}


/// PDFファイルからテキスト内容を抽出する
fn extract_pdf_content(pdf_path: &PathBuf) -> Result<String> {
    // テキストの抽出（直接パスを渡す）
    let text = pdf_extract::extract_text(pdf_path)
        .with_context(|| format!("PDFからのテキスト抽出に失敗しました: {:?}", pdf_path))?;

    Ok(text)
}

/// 抽出したPDFコンテンツをMarkdownに変換する
fn convert_to_markdown(content: String) -> Result<String> {
    // PDFから抽出したテキストを解析して構造を把握
    let mut markdown = String::new();
    let mut lines = content.lines().peekable();

    // 見出しと段落を識別するための正規表現
    let heading_regex = Regex::new(r"^\s*(\d+\.\s+|#+\s+)?(.+)$").unwrap();

    // 前の行のフォントサイズや太さなどを格納する変数（実際のPDF解析では必要になる可能性があります）
    let mut current_block_type = "p"; // デフォルトは段落

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            markdown.push_str("\n\n");
            continue;
        }

        // 見出しの検出（単純化した実装）
        if let Some(caps) = heading_regex.captures(trimmed) {
            let prefix = caps.get(1).map_or("", |m| m.as_str());
            let text = caps.get(2).map_or(trimmed, |m| m.as_str());

            // 数字+ドットで始まるか、大きなフォントサイズの場合は見出しと推定
            if prefix.contains('.') || is_likely_heading(trimmed) {
                let heading_level = determine_heading_level(prefix, trimmed);
                markdown.push_str(&format!("{} {}\n\n", "#".repeat(heading_level), text));
                current_block_type = "h";
                continue;
            }
        }

        // 強調などの書式の検出と変換
        let formatted_line = detect_and_format(trimmed);

        // 段落の処理
        if current_block_type == "p" {
            // 継続する段落かどうかを判断
            if !markdown.ends_with("\n\n") && !markdown.is_empty() {
                markdown.push(' ');
            }
            markdown.push_str(&formatted_line);
        } else {
            markdown.push_str(&formatted_line);
            markdown.push_str("\n\n");
            current_block_type = "p";
        }
    }

    Ok(markdown)
}

/// 行が見出しである可能性を判定（単純化）
fn is_likely_heading(line: &str) -> bool {
    // この実装は単純化しています。実際はPDFのフォントサイズ等を見る必要があります
    line.len() < 100 && !line.ends_with(".") && !line.contains(",")
}

/// 見出しレベルを決定（単純化）
fn determine_heading_level(prefix: &str, text: &str) -> usize {
    // この実装は単純化しています。実際はPDFの階層構造を見る必要があります
    if prefix.starts_with("1.") {
        1
    } else if prefix.starts_with("1.1") || prefix.starts_with("2.") {
        2
    } else if text.len() < 30 && text.to_uppercase() == text {
        1 // 短くて全て大文字の場合はH1と推定
    } else {
        3
    }
}

/// テキスト内の強調などの書式を検出してMarkdown形式に変換
fn detect_and_format(text: &str) -> String {
    // この実装は単純化しています。実際はPDFのスタイル情報を見る必要があります
    // ここでは仮に、全て大文字のワードを強調（太字）とする
    let mut result = String::new();
    let words = text.split_whitespace();

    for word in words {
        if word.to_uppercase() == word && word.len() > 1 && word.chars().any(char::is_alphabetic) {
            result.push_str(&format!("**{}** ", word));
        } else {
            result.push_str(&format!("{} ", word));
        }
    }

    result.trim().to_string()
}

/// Markdownをファイルに書き込む
fn write_to_file(path: &PathBuf, content: &str) -> Result<()> {
    let mut file = File::create(path)
        .with_context(|| format!("出力ファイルの作成に失敗しました: {:?}", path))?;

    file.write_all(content.as_bytes())
        .with_context(|| "ファイルへの書き込みに失敗しました")?;

    Ok(())
}
