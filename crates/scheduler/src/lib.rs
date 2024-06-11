use tauri::AppHandle;
use tauri_plugin_notification::{NotificationExt, PermissionState};
use tokio::time::{interval, Duration};

// todo: check if some deck has an available flashcard to review
//       and send an OS notification.
pub async fn setup_scheduler(app: AppHandle) {
    let notify = app.notification();
    if notify.permission_state().unwrap() != PermissionState::Granted {
        notify.request_permission().unwrap();
    }
    if notify.permission_state().unwrap() == PermissionState::Granted {
        notify
            .builder()
            .title("Memory Up")
            .body("Vamos estudar? Flashcards estão disponíveis para revisão!")
            .show()
            .unwrap();
    }

    let mut interval = interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        // println!("running this every 5 seconds");
    }
}
