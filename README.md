<h2 align="center">CGU Notice Notifier</h2>

<p align="center">Telegram bot which sends notifications to all subscribed users whenever CGU posts any update on their website <a href="https://cgu-odisha.ac.in/notice/">notice board</a>. </p>

<br>
<h3>🛠️ Building from source</h3>

1. Clone the repo
```sh
git clone https://github.com/prashantrahul141/CGU-Notices-Notifier
```

2. Add the following env vars to `.env`
```sh
SITE_URL="https://cgu-odisha.ac.in/notice"
DB_USERNAME=""
DB_PASSWORD=""
DB_CONNECTION_URI=""
DATABASE_NAME="cgu-notice-db"
NOTICES_COLLECTION_NAME="all-notices-col"
METADATA_COLLECTION_NAME="metadata-col"
TELOXIDE_TOKEN=""
```

3. Build and run using cargo
```sh
cargo run --release
```