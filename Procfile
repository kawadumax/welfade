# 開発環境としてforeman startで起動できるようにしているが、docker-composeのほうで開発環境を作るのを推奨
back: cd backend && systemfd --no-pid -s http::3000 -- cargo watch -x run # ホットリロードが効く起動方法
front: cd frontend && yarn serve --port 8080
db: cd docker && docker-compose up db