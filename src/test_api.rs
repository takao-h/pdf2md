use std::path::PathBuf;

fn main() {
    // PDFファイル名が用意されていると仮定
    let file_path = PathBuf::from("example.pdf");
    
    // 方法1: ファイルからバイト列を読み取り、それを渡す
    let bytes = std::fs::read(&file_path).unwrap();
    let text1 = pdf_extract::extract_text(&bytes);
    
    // 方法2: 直接ファイルパスを渡す方法があれば
    // let text2 = pdf_extract::extract_text_from_file(&file_path);
    
    println!("サポートされる機能を確認中");
}
