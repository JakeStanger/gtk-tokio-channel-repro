use std::time::Duration;
use gtk::prelude::*;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let app = gtk::Application::builder().build();

    app.connect_activate(|app| {
        gtk::ApplicationWindow::new(app);

        let (tx, mut rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let mut i = 0;
            loop {
                tx.send(i).await.unwrap();
                println!("SEND [{i}] {}/{}", tx.capacity(), tx.max_capacity());
                sleep(Duration::from_millis(100)).await;

                i += 1;
            }
        });

        glib::spawn_future_local(async move {
            while let Some(i) = rx.recv().await {
                println!("RECV [{i}]");
            }

            println!("RECV END");
        });
    });

    app.run();
}
