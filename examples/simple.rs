extern crate winrt_notification;
use winrt_notification::{Duration, Sound, Toast};

fn main() {
    Toast::new(winrt_notification::POWERSHELL_APP_ID)
        .title("Look at this flip!")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .on_activated(|| {})
        .on_dismissed(|_reason| {})
        .show()
        .expect("unable to send notification");
}
