// Propably there are conflicts for index of chars and bytes in strings... well that's bad

// All functions related to it need to be converted from the outside to non html letters
pub fn convert_plain_to_html(plain: &mut String) {
    //println!("Calling convert_plain_to_html: {}", plain);
    // Why the fuck does it add
    let temp = plain.clone();
    plain.clear();
    html_escape::decode_html_entities_to_string(temp, plain);
    // ?
    *plain = plain.replace("\n", "");
    *plain = plain.replace("\\n", "");
}

pub fn get_word_before_char(html: &String, index: i32) -> String {
    let mut word: String = String::new();
    let chars_vec: Vec<char> = html.chars().collect();
    //println!("For position index: {}", index);

    let mut counter = 0;
    loop {
        let final_post: isize = (index - counter) as isize;
        let character = chars_vec[final_post as usize];
        //println!("At position {} character is: {:?}", final_post, character);
        if character != ' ' {
            word.push(character);
            counter += 1;
            if final_post == 0 {
                break;
            }
        } else {
            break;
        }
    }
    // https://stackoverflow.com/questions/27996430/reversing-a-string-in-rust
    word = word.chars().rev().collect::<String>();
    println!("Found word: {}", word);
    word
}

pub fn stupid_rust_string_index(html: &String, to_find: String) -> Vec<i32> {
    let mut the_veck: Vec<char> = Vec::new();
    let mut do_char_first = true;
    let mut index_count: Vec<i32> = Vec::new();
    let mut counter = 0;
    for byte in html.chars() {
        if the_veck.len() == to_find.len() {
            the_veck.remove(0);
        }
        the_veck.push(byte);
        let str: String = the_veck.clone().into_iter().collect();
        if str == to_find {
            index_count.push(counter);
            println!("Found string index at: {}", counter);
        }
        counter += 1;
    }
    index_count
}

pub fn find_valid_byte_position(html: &String, pos: usize) -> usize {
    let possible_chars: Vec<char> = vec!['<', '>'];
    let mut offset: usize = 0;
    loop {
        let mut exit_it = 0;

        let final_plus = pos + offset;
        if final_plus < html.len()
            && html.is_char_boundary(final_plus as usize)
            && possible_chars.contains(&html.chars().nth(final_plus as usize).unwrap())
        {
            return final_plus;
        } else {
            exit_it += 1;
        }

        let final_minus = pos - offset;
        if final_minus as usize > 0 {
            if html.is_char_boundary(final_minus as usize)
                && possible_chars.contains(&html.chars().nth(final_minus as usize).unwrap())
            {
                return final_minus;
            }
        } else {
            exit_it += 1;
        }

        if exit_it == 2 {
            break;
        }

        offset += 1;
    }
    println!("We are in trouble");
    0
}

// This function doesn't check if the byte offset is correct
// This doesn't solve the issue...
/*
pub fn insert_at_str(main: String, insert: String, index: usize) -> String{
    let the_split = main.split_at(index);
    println!("split... {:?} \n \n \n second: {:?}", the_split.0, the_split.1);
    format!("{}{}{}", the_split.0, insert, the_split.1)
}
*/

pub fn highlight_html_code(html: String, plain: String) -> String {
    let mut final_highlight = html.clone();
    let start_pure_high = "<b>";
    let stop_pure_high = "</b>";
    let find_str_after_text = String::from("</");
    let index: Vec<i32> = stupid_rust_string_index(&html, find_str_after_text.clone());
    println!("Pure index: {:?}", index);
    let mut offset = 0;
    for pos in index {
        // match_indices fucks here some errors
        println!("offset is: {}", offset);
        let pos_real = pos - 2 as i32; // </ it's TODO: maybe /2
        let word_left = &get_word_before_char(&html, pos_real);
        println!("Word left: {} at {}", word_left, pos_real);
        if plain.contains(word_left) {
            // TODO: this is not ideal
            // TODO: test +1
            let very_final_pos =
                find_valid_byte_position(&html, (pos_real + offset).try_into().unwrap()) + 1;

            println!("Word found, adding at: {} but the real position was: {}", very_final_pos, pos_real + offset);

            // Testing too
            let final_test = html.clone();
            
            // Rewrite insert str, its stupid
            final_highlight.insert_str(very_final_pos, start_pure_high);
            if offset == 3 {
                println!("Test end");
                return final_highlight;
            }
            offset = offset + 3 as i32;
        }
    }

    final_highlight
}
