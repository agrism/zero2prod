use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

impl SubscriberName {
    /// Returns an instance of `SubscriberName` if the input satisfies all
    /// our validation constraints on subscriber names.
    /// It panic otherwise.
    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespaces = s.trim().is_empty();

        // A graph is defined by the Unicode standard as a "user-perceived"
        // character: `ā` is a single grapheme, but it is compose of two characters
        // (`a` and ``).
        //
        // `grapheme` returns an iterator over the graphemes in the input `s`
        // `true`  specifies tht we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = s.graphemes(true).count() > 256;

        // Iterate over all characters in the input `s` to check if any of them matches
        // one of characters in the forbidden array.
        let forbidden_characters = ['/', '(', ')', '"', '>', '<', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespaces || is_too_long || contains_forbidden_characters {
            panic!("{} is not a valid subscriber name.", s)
        } else {
            Self(s)
        }
    }

    pub fn inner(self) -> String {
        // The caller gets ghe inner string,
        // but they do not have a SubscriberName anymore!
        // That's because `inner` takes `self` by value,
        // consuming it according to move semantics
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut str {
        // The caller gets a  mutable reference to the inner string.
        // This allows then to perform *arbitrary* changes to
        // value itself, potentially braking our invariants!
        &mut self.0
    }

    pub fn inner_ref(&self) -> &str {
        // The caller gets a shared reference to the inner string.
        // This gives the caller **rea-only** access,
        // they have no way to compromise our invariants!
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
