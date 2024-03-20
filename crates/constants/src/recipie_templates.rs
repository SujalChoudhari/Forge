use std::collections::HashMap;

pub fn recipe_templates() -> HashMap<&'static str, &'static str> {
    let template: HashMap<&str, &str> = [(
      "cargo_build", r#"
build:
  always: true
  vars: 
  - name : forge
  run:
    - cargo build --release

    - win
    - copy .\\target\\release\\{name}.exe .\\{name}_{version}_win.exe
    - Get-FileHash .\\{name}_{version}_win.exe > .\\sha256_{version}_win.txt

    - mac
    - cp ./target/release/{name} ./{name}_{version}_mac.so
    - sha256sum ./{name}_{version}_mac.so > ./sha256_{version}_mac.txt

    - linux
    - cp ./target/release/{name} ./{name}_{version}_linux.so
    - sha256sum ./{name}_{version}_linux.so > ./sha256_{version}_linux.txt"#
    ),
        (
            "github_release",
            r#"
github_release:
  always: true
  run:
    - git tag v{version}
    - git push origin v{version}
    - hub release create -a dist/{version}.zip -m "Release {version}"
"#,
        ),
        (
            "lint_code",
            r#"
lint_code:
  detect: ["src/", "*.js", "*.css"]
  always: false
  run:
    - eslint src/
    - stylelint *.css
"#,
        ),
        (
            "test_and_deploy",
            r#"
test_and_deploy:
  always: true
  run:
    - npm test
    - ssh user@server 'cd /path/to/project && git pull origin main && npm install && npm run build'
"#,
        ),
        (
            "update_dependencies_py",
            r#"
update_dependencies_py:
  detect: ["requirements.txt"]
  always: false
  run:
    - pip install -r requirements.txt
"#,
        ),
        (
            "update_dependencies_js",
            r#"
update_dependencies_js:
  detect: ["package.json"]
  always: false
  run:
    - npm update
"#,
        ),
        (
            "docker_build_and_push",
            r#"
docker_build_and_push:
  detect: ["Dockerfile"]
  run:
    - docker build -t my_image:{version} .
    - docker push my_image:{version}
"#,
        ),
        (
            "deploy_to_staging",
            r#"
deploy_to_staging:
  detect: ["src/", "test/"]
  always: false
  run:
    - npm run build
    - scp -r dist/ user@staging_server:/path/to/deployment
"#,
        ),
        (
            "update_documentation",
            r#"
update_documentation:
  always: true
  run:
    - mkdocs build
    - git add .
    - git commit -m "Update documentation"
    - git push origin main
"#,
        ),
        (
            "backup_database",
            r#"
backup_database:
  detect: ["database/"]
  always: false
  run:
    - mysqldump -u username -p password database_name > backup.sql
    - scp backup.sql user@backup_server:/path/to/backup
"#,
        ),
        (
            "deploy_to_cloud_platform",
            r#"
deploy_to_cloud_platform:
  detect: ["terraform/", "*.tf"]
  always: true
  run:
    - terraform plan
    - terraform apply -auto-approve
"#,
        ),
        (
            "python_fastapi",
            r#"
python_fastapi:
  detect: ["main.py"]
  always: true
  run:
    - python -m venv venv
    - source venv/bin/activate
    - pip install -r requirements.txt
    - uvicorn main:app --reload
"#,
        ),
        (
            "python_flask",
            r#"
python_flask:
  detect: ["app.py"]
  always: true
  run:
    - python -m venv venv
    - source venv/bin/activate
    - pip install -r requirements.txt
    - flask run
"#,
        ),
        (
            "nodejs_express",
            r#"
nodejs_express:
  detect: ["server.js"]
  always: true
  run:
    - npm install
    - npm start
"#,
        ),
        (
            "ruby_rails",
            r#"
ruby_rails:
  detect: ["Gemfile"]
  always: true
  run:
    - bundle install
    - rails server
"#,
        ),
        (
            "go_gin",
            r#"
go_gin:
  detect: ["main.go"]
  always: true
  run:
    - go mod tidy
    - go run main.go
"#,
        ),
        (
            "java_spring_boot",
            r#"
java_spring_boot:
  detect: ["pom.xml"]
  always: true
  run:
    - mvn clean install
    - java -jar target/*.jar
"#,
        ),
        (
            "php_laravel",
            r#"
php_laravel:
  detect: ["composer.json"]
  always: true
  run:
    - composer install
    - php artisan serve
"#,
        ),
        (
            "rust_rocket",
            r#"
rust_rocket:
  detect: ["Cargo.toml"]
  always: true
  run:
    - cargo build
    - cargo run
"#,
        ),
    ]
    .iter()
    .cloned()
    .collect();
    template
}
