// ANCHOR: here
//! # Mening Crateyim
//!
//! `my_crate` - muayyan hisob-kitoblarni bajarishni qulayroq qilish uchun
//! yordamchi dasturlar to'plami.

/// Berilgan raqamga bitta qo'shadi.
// --snip--
// ANCHOR_END: here
///
/// # Misollar
///
/// ```
/// let argument = 5;
/// let javob = my_crate::bir_qoshish(argument);
///
/// assert_eq!(6, javob);
/// ```
pub fn bir_qoshish(x: i32) -> i32 {
    x + 1
}
