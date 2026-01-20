# Architecture & Design Rules

このドキュメントでは、`htup` プロジェクトにおけるアーキテクチャ設計方針を定義します。
本プロジェクトでは **Clean Architecture (Onion Architecture)** を採用し、**Rust** の特性に合わせた厳密なレイヤー分けと依存性注入 (DI) を行います。

## 1. 全体構成 (Workspace)

プロジェクトは Cargo Workspace を以下の構成で使用します。将来的なフロントエンド(WASM/Browser)対応を見据え、Coreロジックを完全に分離します。

```
.
├── core/           # ビジネスロジック、ドメインモデル、ユースケース (Library)
├── tui/            # TUI プレゼンテーション層 (Executable)
└── web/            # (Future) ブラウザ版フロントエンド
```

## 2. Clean Architecture Layers (in `core`)

`core` クレート内では以下のレイヤー構造を厳守してください。依存の方向は **外側 → 内側** (Infrastructure -> UseCase -> Domain) のみ許容されます。

### 2.1 Domain Layer (`core/src/domain/`)
**最も内側のレイヤー。外部への依存を一切持ちません。**

- **Entities**: 純粋なデータ構造 (Struct)。ビジネスルールのみを持つ。
    - `Request` (Method, Url, Headers, Body)
    - `Response` (Status, Body, Duration)
    - `Project`
- **Repository Traits**: データの永続化に関するインターフェース定義。実装はここには置かない。
    - `trait RequestRepository`
    - `trait ProjectRepository`
- **Gateway Traits**: 外部システムとの通信インターフェース。
    - `trait HttpClient`: HTTPリクエスト送受信
    - `trait Editor`: エディタ起動

### 2.2 UseCase Layer (`core/src/usecase/`)
**アプリーケーションのビジネスロジック。Domain層のTraitにのみ依存します。**

- 各ユースケースは単一の責任を持つ Struct/Trait として定義します。
- **Dependency Injection**: 必要な Repository や Gateway は `Arc<dyn Trait>` または Generics として注入されます。

**Example:**
```rust
pub struct ExecuteRequestUseCase<R: RequestRepository, C: HttpClient> {
    repo: Arc<R>,
    client: Arc<C>,
}

impl<R, C> ExecuteRequestUseCase<R, C> {
    pub async fn execute(&self, request_id: &str) -> Result<Response> {
        // Logic: Load request -> Send via Client -> Return Response
    }
}
```

### 2.3 Adapter / Infrastructure Layer (`core/src/infra/`)
**Domain層のTraitに対する具体的な実装。**

- `FsRequestRepository`: ファイルシステムを使った実装。
- `ReqwestHttpClient`: `reqwest` を使った `HttpClient` の実装。
- `SystemCommandEditor`: `std::process::Command` を使った実装。

## 3. TUI Layer (`tui/src/`)
**Coreに対するPresentation層として振る舞います。**

- **Architecture**: MVU (Model-View-Update) または Clean Architecture の Controller/Presenter パターン。
- **Role**:
    - ユーザー入力を受け取り、適切な `UseCase` を呼び出す。
    - `UseCase` からの結果を受け取り、描画用モデルに変換して描画する。
    - **ビジネスロジックは一切持たない。**

## 4. Testability & Rules
1. **Trait First**: すべての外部依存 (File IO, Network, System Calls) は必ず Trait として定義し、Mocking 可能にすること。
2. **Unit Tests**: Domain と UseCase は 100% 単体テスト可能であること。
    - `mockall` クレート等を使用して、Repositoryのモックを作成しテストする。
3. **No Global State**: `lazy_static` や `static mut` によるグローバルステートの共有は禁止。全て DI コンテナ (または `main` での初期化と注入) を通じて渡す。
4. **Error Handling**: 各レイヤーで適切な独自エラー型 (`thiserror` 等) を定義し、詳細を隠蔽しつつ必要な情報を上位に伝える。
