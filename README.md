# Rust 프로그래밍 언어

이것은 표준 라이브러리, 도구 그리고 문서등이 포함된 Rust 컴파일러입니다.

## 빨리 시작하기

### Windows

1. 다운로드 받고 [installer][win-exe]를 이용합니다.
2. [튜토리얼]을 읽습니다.
3. 즐깁니다!

> ***Note:*** 윈도우즈 사용자들은 위키에서 세부적인
> [getting started][wiki-start] 내용을 읽어야합니다.
> 바이너리 인스톨러를 사용하더라도 윈도우즈에서 빌드하려면
> MinGW 설치가 필요합니다.
> 자세한 내용은 여기서 논의하지 않습니다.

[튜토리얼]: http://sarojaba.github.io/rust-doc-korean/doc/tutorial.html
[wiki-start]: https://github.com/mozilla/rust/wiki/Note-getting-started-developing-Rust
[win-exe]: http://static.rust-lang.org/dist/rust-0.8-install.exe

### Linux / OS X

1. (설치되어 있지 않다면) 사전에 필요한 패키지를 설치합니다.

	* g++ 4.4 또는 clang++ 3.x
	* python 2.6 또는 그 이상 (그러나 3.x는 안됨)
	* perl 5.0 또는 그 이상
	* gnu make 3.81 또는 그 이상
	* curl

2. 다운로드 받고 Rust를 빌드합니다.
	[tarball]을 다운로드 받거나 [repo]로부터 직접 빌드할 수 있습니다.

	[tarball]에서 빌드하는 방법

		$ curl -O http://static.rust-lang.org/dist/rust-0.8.tar.gz
        $ tar -xzf rust-0.8.tar.gz
        $ cd rust-0.8
    
	또는 [repo]에서 빌드하는 방법

		$ git clone https://github.com/mozilla/rust.git
        $ cd rust

	Rust의 소스 코드를 구했다면, 구성과 빌드를 할 수 있습니다.

		$ ./configure
		$ make && make install

	만약 목표 디렉토리를 수정할 권한을 가지고 있지 않다면
	`sudo make install`을 사용해야 할 수도 있습니다.
	`configure` 명령의 인자로 `--prefix`를 전달하여
	설치 장소를 조정할 수 있습니다. 다양한 다른 옵션들도 지원되고
	`--help` 명령을 통해 더 자세한 정보를 얻을 수 있습니다.

	`make install`이 성공적으로 완료되면
	`/usr/local/bin` 디렉토리 내의 몇가지 프로그램을 볼 수 있을 것입니다.
	`rustc`는 the Rust 컴파일러, `rustdoc`는 API-문서 도구,
	그리고 `rustpkg`는 Rust 패키지 관리자 및 빌드 시스템입니다.

3. [튜토리얼]을 읽습니다.
4. 즐깁니다!

[repo]: https://github.com/mozilla/rust
[tarball]: http://static.rust-lang.org/dist/rust-0.7.tar.gz
[튜토리얼]: http://sarojaba.github.io/rust-doc-korean/doc/tutorial.html

## 더보기

현재 Rust 컴파일러는 [tarball]로 제작되고, Windows의 경우는 [installer][win-exe] 사용을 추천합니.

Rust 컴파일러는 Rust로 작성되었기에, 스스로 미리 컴파일된 "snapshot" 버전으로 만들어집니다(개발의 초기 단계에). 이처럼, 소스를 빌드하려면 snapshots을 받아오기 위해 인터넷 연결과, 유효한 스냅샷 바이너리를 실행할 수 있는 OS가 필요합니다.

현재 스냅샷 바이너리는 아래의 플랫폼들에서 제작되고 테스트됩니다:

* Windows (7, Server 2008 R2), x86 only
* Linux (various distributions), x86 and x86-64
* OSX 10.6 ("Snow Leopard") or greater, x86 and x86-64

다른 플랫폼에서 작동하는 것을 찾을 수도 있지만, 선호되는 빌드 환경을 지원하는 것이 우선의 목표입니다.

현재 Rust는 swapping 없이 빌드하기 위해 약 1.8G의 RAM을 필요로 합니다.
만약 swap이 사용된다면, 빌드하는데 아주 오랜 시간이 걸릴것입니다.

[wiki]에 문서들이 더 많이 있습니다.

[wiki]: https://github.com/mozilla/rust/wiki

## 라이선스

Rust는 MIT 라이선스와 Apache 라이선스 (버전 2.0)의 계약 조건에 따라 우선적으로 배포되고, 일부분은 여러가지의 BSD 계열의 라이선스를 따릅니다.

LICENSE-APACHE, LICENSE-MIT, 그리고 COPYRIGHT 자세히 보시길 바랍니다.
