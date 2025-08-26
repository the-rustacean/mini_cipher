# mini cipher

## 폴더 구조
```sh
/mini_cipher
    README.md
    cargo.toml
    plain.txt
    /src
        lib.rs
        main.rs
```

## 실행 및 옵션
### key: number
### mode: encoding | decoding
### file_path:
```sh
$ cargo run -- 13 encoding plain.txt
```

## 환경변수 추가
```sh
$ $Env:IGNORE_DESC=1; cargo run -- 13 encoding plain.txt
```

## 환경변수 제거
```sh
$ Remove-Item Env:IGNORE_DESC
```

## 출력결과를 파일로 저장
```sh
$ cargo run -- 13 encoding plain.txt > output.txt
```