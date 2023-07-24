# Utilisez une image de Rust comme image de base
FROM rust:latest as build

# Installez les dépendances de développement nécessaires pour votre projet
RUN apt-get update && apt-get install -y libpq-dev

# Créez un dossier pour votre application dans l'image Docker
RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

# Copiez les fichiers de votre application dans l'image Docker
COPY . .

# Exécutez la commande de build pour compiler votre application en mode release
RUN cargo build --release

# Copiez l'exécutable compilé à partir de l'étape précédente dans cette nouvelle image

# Exposez le port sur lequel l'application écoute
EXPOSE 8080 

# Définissez le point d'entrée pour exécuter votre application lorsque le conteneur démarre
CMD ["./target/release/portfolio-api"]