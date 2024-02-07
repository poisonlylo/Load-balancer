use std::net::TcpListener;
use std::thread;

mod proxy;
mod config;

fn main() {
    // Gestion des arguments de la ligne de commande
    let options = config::CmdOptions::parse();
    
    // Initialisation de l'état du proxy
    let proxy_state = proxy::ProxyState::new(&options);

    // Démarrage du serveur TCP
    let listener = match TcpListener::bind(&options.bind) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Could not bind to {}: {}", options.bind, err);
            std::process::exit(1);
        }
    };
    println!("Listening for requests on {}", options.bind);

    // Acceptation des connexions entrantes
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Copie de l'état du proxy pour chaque thread
        let state = proxy_state.clone();

        // Gestion de la connexion dans un thread séparé
        thread::spawn(|| {
            proxy::handle_connection(stream, &state);
        });
    }
}
