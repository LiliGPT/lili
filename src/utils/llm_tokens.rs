pub fn llm_tokens_count_string(text: &str) -> usize {
    use tiktoken_rs::p50k_base;

    let bpe = p50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(text);
    // println!("Token count: {}", tokens.len());
    tokens.len()
}
