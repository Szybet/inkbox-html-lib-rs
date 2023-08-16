use core::num;

// html
use regex::Regex;

// A CPP wrapper is needed for this function
// plain_highlights is list of plain non html highlights splitted by U+001F ( Literally this string ) which is https://symbl.cc/en/001F/
// This function returns a new updated main_page
// Pages can be empty, indicating they don't exist

pub fn purge_html(html: String) -> String {
    let regex: Regex = Regex::new(r"<.*?>").unwrap();
    regex.replace_all(&html, "").to_string()
}

// From left
pub fn diffrence_in_string_left(longer_string: String, other_string: String) -> String {
    let mut final_string = String::new();
    let longer_words: Vec<&str> = longer_string.split_whitespace().collect();
    let words: Vec<&str> = other_string.split_whitespace().collect();

    let num_diffrence = longer_words.len() - words.len() - 2; // words were cutted out? len starts from 1
                                                              // Maybe a better way?
    for word in num_diffrence..longer_words.len() {
        final_string.push_str(longer_words[word]);
        final_string.push(' ');
    }

    final_string.pop(); // space
    println!("The diffrence text is ( left ): {}", final_string);

    final_string
}

// From right ( previous page )
// Weird but works?
pub fn diffrence_in_string_right(longer_string: String, other_string: String) -> String {
    let mut final_string = String::new();
    let longer_words: Vec<&str> = longer_string.split_whitespace().collect();
    let words: Vec<&str> = other_string.split_whitespace().collect();

    let num_diffrence = longer_words.len() - words.len(); // words were cutted out? len starts from 1
                                                              // Maybe a better way?
    for word in (0..num_diffrence).rev() {
        final_string.insert_str(0, longer_words[word]);
        final_string.insert(0, ' ');
    }

    //final_string.pop(); // space
    println!("The diffrence text is ( right ): {}", final_string);

    final_string
}

pub fn highlight_page(
    main_page: String,
    previous_page: String,
    next_page: String,
    plain_highlights: String,
) -> String {
    let mut converted_main_page = main_page;
    convert_plain_to_html(&mut converted_main_page);
    let mut converted_previous_page = previous_page;
    convert_plain_to_html(&mut converted_previous_page);
    let mut converted_next_page = next_page;
    convert_plain_to_html(&mut converted_next_page);
    // To remove additionall lines
    let mut converted_plain_highlights = plain_highlights;
    convert_plain_to_html(&mut converted_plain_highlights);

    for highlight in converted_plain_highlights.split("U+001F") {
        println!("Next highlight...");
        let result = find_text_in_html_code(&converted_main_page, highlight.to_string());

        println!("The result of find_text_in_html_code: {:?}", result);

        if let Some(html_highlight) = result.0 {
            // Its cutted to the next page
            if result.1 == true && !converted_next_page.is_empty() {
                println!("One highlight is cutted to the next page");
                // Check for it
                // I hope that only full purge is needed
                let clean_highlight = purge_html(highlight.to_string());
                let clean_html_highlight = purge_html(html_highlight.to_string()); // Not so html anymore
                println!("clean_highlight: {}", clean_highlight);
                println!("clean_html_highlight: {}", clean_html_highlight);
                let diffrence = diffrence_in_string_left(clean_highlight, clean_html_highlight);

                let pure_next_page = purge_html(converted_next_page.clone());
                let diffrence_vec: Vec<&str> = diffrence.split_ascii_whitespace().collect();
                let pure_next_page_vec: Vec<&str> =
                    pure_next_page.split_ascii_whitespace().collect();
                let mut counter = 0;
                let mut identical = true;
                for word in diffrence_vec.iter() {
                    if word != &pure_next_page_vec[counter] {
                        println!("It's not identical");
                        identical = false;
                        break;
                    }
                    counter += 1;
                }
                if identical == false {
                    println!("This highlight is cutted but doesn't appear on the next page");
                    break;
                } else {
                    println!("The highlight is on the next page! that's good");
                }
            }
            // Its cutted to the previous page
            if result.2 == true {
                println!("One highlight is cutted to the previous page");
                let clean_highlight = purge_html(highlight.to_string());
                let clean_html_highlight = purge_html(html_highlight.to_string()); // Not so html anymore
                println!("clean_highlight: {}", clean_highlight);
                println!("clean_html_highlight: {}", clean_html_highlight);
                let diffrence = diffrence_in_string_right(clean_highlight, clean_html_highlight);

                let pure_previous_page = purge_html(converted_previous_page.clone());
                let diffrence_vec: Vec<&str> = diffrence.split_ascii_whitespace().collect();
                let pure_previous_page_vec: Vec<&str> =
                pure_previous_page.split_ascii_whitespace().collect();
                let mut counter = pure_previous_page_vec.len() - 1;
                let mut identical = true;
                for word in diffrence_vec.iter().rev() {
                    if word != &pure_previous_page_vec[counter] {
                        println!("It's not identical");
                        identical = false;
                        break;
                    }
                    counter -= 1;
                }
                if identical == false {
                    println!("This highlight is cutted but doesn't appear on the previous page");
                    break;
                } else {
                    println!("The highlight is on the previous page! that's good");
                }

            }
            let html_highlight_done = highlight_html_code(&html_highlight);
            converted_main_page =
                converted_main_page.replace(&html_highlight, &html_highlight_done);
        }
    }
    converted_main_page
}

// All functions related to it need to be converted from the outside to non html letters
pub fn convert_plain_to_html(plain: &mut String) {
    //println!("Calling convert_plain_to_html: {}", plain);
    // Why the fuck does it add the text
    let temp = plain.clone();
    plain.clear();
    html_escape::decode_html_entities_to_string(temp, plain);
    // Spaces because highlighting in plain, otherwise isin't important
    *plain = plain.replace("\n", " ");
    *plain = plain.replace("\\n", " ");
}

pub fn highlight_html_code(html: &String) -> String {
    let mut final_highlight = html.clone();
    let start_pure_high = "<b>";
    let stop_pure_high = "</b>";
    let text_ends_at = "</span>";

    // Regex...
    // <span style=[^>]*>
    let re = Regex::new(r"<span style=[^>]*>").unwrap();
    let mut span = String::new();
    // All fonts / captures are the the same
    if let Some(captures) = re.captures(&final_highlight) {
        // Use indexing (1) to access the first capture group value
        if let Some(first_capture) = captures.get(0) {
            span = first_capture.as_str().to_string();
            println!("First captured value: {}", span);
        }
    }
    let start_value = &format!("{}{}", span, start_pure_high);
    println!("start_value: {}", start_value);
    let end_value = &format!("{}{}", stop_pure_high, text_ends_at);
    println!("end_value: {}", end_value);
    if !span.is_empty() {
        // It fits in one html tag
        final_highlight = final_highlight.replace(&span, start_value);
    }

    final_highlight = final_highlight.replace(text_ends_at, end_value);

    format!("{}{}{}", start_pure_high, final_highlight, stop_pure_high)
}

// Finds plain text in html code even if there are tags between it, and outputs the text in the html code that contains those tags
// return arguments:
// String of the html text,
// if it's cutted off ( to the next page )
// if it's cutted off ( to the previous page ) ( propably? )
pub fn find_text_in_html_code(
    html_code: &String,
    plain_text: String,
) -> (Option<String>, bool, bool) {
    let plain_text_split_whitespace: Vec<&str> = plain_text.split_whitespace().collect();
    let mut start: usize = 0;
    let mut end: usize;
    let mut list_start: Vec<usize> = Vec::new();
    let mut conflict: bool = false; // Variable used inside start loop
    let mut conflict_resolved = false;
    let mut previous_page_cutted_off = false;
    let mut no_conflict = false;
    // For previous support
    let mut false_strings: Vec<String> = Vec::new(); // If `Final smallest diffrence:` didn't changed, put this word here to ignore it & re run the loop
    let mut first_word_str_prev: String = String::new();
    let mut exit_inf_loop = false;
    // Looking for start
    loop {
        for word in plain_text_split_whitespace.clone() {
            if false_strings.contains(&word.to_string()) { // wtf &String
                println!("Skipping the first word: we maybe need to look at previous page?");
                continue;
            }
            if false_strings.len() >= plain_text_split_whitespace.len() {
                println!("We checked for all words: there are none valid, no highlight on this page...");
                return (None, false, false);
            }
            // HTML space
            //println!("the word: {}", word);
            let index: Vec<(usize, &str)> = html_code.match_indices(word).collect();
            if index.len() == 0 && conflict == false {
                println!("previous_page_cutted_off set to true");
                previous_page_cutted_off = true;
                continue;
            }
            if index.len() > 1 {
                for num in index {
                    if conflict == false {
                        println!("The first word is: {} at byte {}", num.1, num.0);
                        list_start.push(num.0);
                        first_word_str_prev = num.1.to_string();
                    }
                }
                conflict = true;
                println!("There is a conflict");
            } else if conflict == false {
                println!("Starting word without conflict: {}", word);
                let first_word = index.first();
                if let Some(first_word_solution) = first_word {
                    start = first_word_solution.0;
                    no_conflict = true;
                    conflict = false;
                    exit_inf_loop = true;
                    break;
                } else {
                    println!("There is no such word... continue ( previous page? )");
                    conflict = false; // Reset
                    false_strings.push(first_word_str_prev.clone());
                    continue;
                }
                //start = index.first().unwrap().0;

            } else {
                // In conclusion, we are here because while iterating, we finally found a match.
                // Now we need to conclude this match with items in list_start, which one fits the best ( is to the nearest left )

                // The nearest in start? to the left

                if index.first().is_none() {
                    // Not sure about this one
                    conflict = false; // Only to reset
                    false_strings.push(first_word_str_prev.clone());
                    break;
                }
                let final_index = index.first().unwrap().0 as isize;

                let static_num = 999999;
                let mut smallest_diffrence = static_num;
                println!("Comparing first words, final index is: {}", final_index);
                for position in list_start.clone() {
                    let calc = final_index - position as isize;
                    println!("calc: {}", calc);
                    println!("position: {}", position);
                    if calc > 0 && calc < smallest_diffrence {
                        smallest_diffrence = calc;
                        start = position;
                    }
                }
                if start as isize != static_num {
                    println!("Final smallest diffrence: {}", smallest_diffrence);
                    conflict = false;
                    exit_inf_loop = true;
                    conflict_resolved = true;
                    break;
                } else {
                    conflict = false; // Only to reset
                    false_strings.push(first_word_str_prev.clone());
                    break;
                }
            }
        }
        if exit_inf_loop == true {
            break;
        }
    }

    if conflict == true {
        println!("There are many duplicate texts...");
        start = list_start.first().unwrap().clone();
    } else if conflict_resolved == true {
        println!("Conflict was resolved");
    } else if no_conflict == true {
        println!("There was no conflict");
    } else {
        println!("There is nothing?");
        return (None, false, false);
    }
    println!("Index of start is: {}", start); // Bytes
    let html_code_from_start_string: &str = html_code.split_at(start).1;
    let html_code_from_start: Vec<&str> = html_code_from_start_string.split_whitespace().collect();
    //println!("Char vec: {:?}", html_code_from_start);
    let last_word = plain_text_split_whitespace.last().unwrap().clone();
    println!("Last word is: {}", last_word);

    // TODO: account for many the same last words...
    let mut count_bytes = 0;
    let mut found_word = false;
    // Together they make a mess
    let regex: Regex = Regex::new(r"<.*?>").unwrap();
    // This catches unclosed tags until a whitespace, can this be a problem?
    let regex2: Regex = Regex::new(r"<.*\S").unwrap();
    // Again, but in the other direction: >
    let regex3: Regex = Regex::new(r".*>").unwrap();
    for kinda_word in html_code_from_start {
        //println!("kinda_word: {}", kinda_word);
        count_bytes += kinda_word.len() + 1;
        if kinda_word.contains(last_word) {
            println!(
                "Last word contains match: '{}' for '{}'",
                kinda_word, last_word
            );
            let mut word_not_html = regex.replace_all(kinda_word, "");

            // Edge cases, repaired by more regex
            let tmp_wnh = word_not_html.clone();
            word_not_html = regex2.replace_all(&tmp_wnh, "");

            let tmp_wnh2 = word_not_html.clone();
            word_not_html = regex3.replace_all(&tmp_wnh2, "");

            println!("Cleaned kinda_word from html: {}", word_not_html);
            // The white space is the problem. so:
            if tmp_wnh != word_not_html {
                // Locate if there is problem with that ( unclosed html tag... )
                count_bytes = count_bytes - (tmp_wnh.len() - word_not_html.len());
                println!("Found last word match, but problematic");
                found_word = true;
                break;
            }
            if word_not_html == last_word {
                println!("Found last word match");
                found_word = true;
                break;
            }
        }
    }
    // End is relative
    let mut end = count_bytes;
    end -= 1;

    // Not sure, but does this need to be always on?
    if found_word == false {
        // The text is cutted off?
        // TODO?
        println!("End word not found?");
    }

    println!("End is at: {}", end);

    let final_string = html_code_from_start_string.split_at(end).0;
    println!("The final thing: {:?}", final_string);

    return (
        Some(final_string.to_owned()),
        !found_word,
        previous_page_cutted_off,
    );
}
