extern crate winrt_notification;
use winrt_notification::{Duration, Sound, Toast};

fn main() {
    let duration = Duration::Short;
    let sound = Some(Sound::SMS);

    let toast = Toast::new(winrt_notification::POWERSHELL_APP_ID)
        .title("first toast")
        .text1("line1")
        .duration(duration)
        .sound(sound)
        .on_activated(|| {})
        .on_dismissed(|_reason| {});

    toast
        .show()
        // silently consume errors
        .expect("notification failed");

    toast
        .show()
        // silently consume errors
        .expect("notification failed");
}
