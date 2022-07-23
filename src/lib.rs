use core::fmt::Display;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

type Message = Vec<Letter>;
type Letter = Vec<Pulse>;

enum Pulse {
    Long,
    Short
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            Pulse::Long => write!(f, "."),
            Pulse::Short => write!(f, "_"),
        }
    }
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message { 
        use Pulse::*;
        let mut msg = Vec::with_capacity(self.len());

        for c in self.chars() {
            let pulses = match c {
                'A' | 'a' => vec![Short, Long],
                'B' | 'b' => vec![Long, Short, Long, Short],
                _ => continue,
            };
            msg.push(pulses);
        }

        msg
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        println!(" ");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
