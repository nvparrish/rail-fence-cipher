#![crate_name = "rail_fence_cipher"]
//! A library for implementing a rail fence cipher
//!
//! The rail-fence cipher is based on setting a number of tracks.  To encode
//! a message, the letters are placed on rails, one at a time, down the set
//! of tracks, then back up.  So for a four-track pattern, it could have the
//! following structure:
//!
//! R - - - - - G - - - - - - - - - - 
//! - U - - - S - R - - - - - - - - - 
//! - - S - I - - - E - T - - - - - - 
//! - - - T - - - - - A - - - - - - - 
//!
//! Then the letters are read in order on each line
//! RGUSRSIETTA
//!
//! To decode, the letters must be arranged on the rails and read in the
//! zig-zag fence pattern again.

/// Rail fence structure
///
/// This structure holds information pertaining to the rail fence cipher
pub struct RailFence {
    /// The number of rails
    rails: u32,
}

impl RailFence {
    /// Create a new fence with the specified number of rails
    ///
    /// # Arguments
    /// * `rails`   The number of rails for this cipher
    pub fn new(rails: u32) -> RailFence {
        RailFence {rails}
    }

    /// Encode the message in text using the fence rails
    ///
    /// # Arguments
    ///
    /// * `text`    The clear-text string to encode
    ///
    /// # Returns
    /// The cipher-text message
    ///
    /// # Example
    /// ```
    /// use rail_fence_cipher::*;
    /// let cipher = RailFence::new(4);
    /// let cipher_text = cipher.encode("RUSTISGREAT");
    /// let expected = String::from("RGUSRSIETTA");
    /// assert_eq!(expected, cipher_text)
    /// ```
    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![String::from(""); self.rails as usize];
        let mut f:usize = 0;
        let mut increment:i32 = 1;
        let message = String::from(text);
        for c in message.chars() {
            rails[f].push(c);
            if increment.is_negative(){
                f -= 1;
            } else {
                f += 1;
            }
            if f == (self.rails-1) as usize {
                increment = -1;
            } else if f == 0 {
                increment = 1;
            }
        };
        let mut result = String::new();
        for part in rails {
            result.push_str(part.as_str());
        }
        result
    }

    /// Encode the message in text using the fence rails
    ///
    /// # Arguments
    ///
    /// * `text`    The clear-text string to encode
    ///
    /// # Returns
    /// The cipher-text message
    ///
    /// # Example
    /// ```
    /// use rail_fence_cipher::*;
    /// let cipher = RailFence::new(4);
    /// let clear_text = cipher.decode("RGUSRSIETTA");
    /// let expected = String::from("RUSTISGREAT");
    /// assert_eq!(expected, clear_text)
    /// ```
    pub fn decode(&self, cipher: &str) -> String {
        if self.rails == 1 {
            return String::from(cipher)
        }
        let mut rails = vec![String::from(""); self.rails as usize];
        let mut start = vec![0_usize; self.rails as usize];
        let cipher_text = String::from(cipher);

        // Identify the start of each new row
        let period = 2 * (self.rails - 1);
        let cipher_length = cipher_text.chars().count();
        let section = cipher_length as u32 / period;
        let remainder = cipher_length as u32 % period;
        start[0] = 0;
        if self.rails > 1 {
            if remainder > 0 {
                start[1] = (section + 1) as usize;
            } else {
                start[1] = section as usize;
            }
        } else {
            return cipher_text; // Simple to decode in the clear
        }
        for i in 2..(self.rails as usize) {
            start[i] = start[i-1] + 2 * section as usize;
            if remainder > (i-1) as u32 {
                start[i] += 1;
            }
            if remainder + (i-1) as u32 + 1 >= 2 * self.rails {
                start[i] += 1;
            }
        }

        // Split the data into the corresponding rows
        for (i, it) in rails.iter_mut().enumerate() {
            if (self.rails - 1) as usize == i {
                it.push_str(&cipher_text[start[i]..]);
            } else {
                it.push_str(&cipher_text[start[i]..start[i+1]]);
            }
        }

        // Pull off letters in the zig-zag pattern to make the decoded message
        let mut clear_text = String::from("");
        let mut f:u32 = 0;
        let mut increment:i32 = 1;
        while !rails[f as usize].is_empty() {
            let ch = rails[f as usize].remove(0);
            clear_text.push(ch);
            if increment.is_negative() {
                f -= 1;
            } else {
                f += 1;
            }
            if f == 0 {
                increment = 1;
            } else if f == (self.rails - 1) {
                increment = -1;
            }
        }
        clear_text
    }
}
