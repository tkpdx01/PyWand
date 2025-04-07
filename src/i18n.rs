use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Language {
    English,
    Chinese,
    Japanese,
    Korean,
    French,
    German,
    Russian,
}

impl Default for Language {
    fn default() -> Self {
        detect_system_language()
    }
}

pub fn detect_system_language() -> Language {
    let lang = env::var("LANG")
        .or_else(|_| env::var("LC_ALL"))
        .or_else(|_| env::var("LANGUAGE"))
        .unwrap_or_else(|_| "en_US.UTF-8".to_string());
    
    let lang_code = lang.split('.').next().unwrap_or("en_US");
    let lang_prefix = lang_code.split('_').next().unwrap_or("en");
    
    match lang_prefix {
        "zh" => Language::Chinese,
        "ja" => Language::Japanese,
        "ko" => Language::Korean,
        "fr" => Language::French,
        "de" => Language::German,
        "ru" => Language::Russian,
        _ => Language::English,
    }
}

macro_rules! translation_map {
    ($($key:expr => {
        en: $en:expr,
        zh: $zh:expr,
        ja: $ja:expr,
        ko: $ko:expr,
        fr: $fr:expr,
        de: $de:expr,
        ru: $ru:expr
    }),* $(,)?) => {
        {
            let mut map = HashMap::new();
            $(
                let mut inner_map = HashMap::new();
                inner_map.insert(Language::English, $en);
                inner_map.insert(Language::Chinese, $zh);
                inner_map.insert(Language::Japanese, $ja);
                inner_map.insert(Language::Korean, $ko);
                inner_map.insert(Language::French, $fr);
                inner_map.insert(Language::German, $de);
                inner_map.insert(Language::Russian, $ru);
                map.insert($key, inner_map);
            )*
            map
        }
    };
}

lazy_static! {
    static ref TRANSLATIONS: HashMap<&'static str, HashMap<Language, &'static str>> = translation_map! {
        // 通用
        "app_name" => {
            en: "PyWand - Python Dependency Analyzer",
            zh: "PyWand - Python依赖分析器",
            ja: "PyWand - Python依存関係アナライザー",
            ko: "PyWand - Python 종속성 분석기",
            fr: "PyWand - Analyseur de dépendances Python",
            de: "PyWand - Python-Abhängigkeitsanalysator",
            ru: "PyWand - Анализатор зависимостей Python"
        },
        "what_to_do" => {
            en: "What would you like to do?",
            zh: "您想做什么？",
            ja: "何をしたいですか？",
            ko: "무엇을 하시겠습니까?",
            fr: "Que souhaitez-vous faire ?",
            de: "Was möchten Sie tun?",
            ru: "Что вы хотите сделать?"
        },
        "local_development" => {
            en: "Local Development",
            zh: "本地开发",
            ja: "ローカル開発",
            ko: "로컬 개발",
            fr: "Développement local",
            de: "Lokale Entwicklung",
            ru: "Локальная разработка"
        },
        "export_offline" => {
            en: "Export for Offline Development",
            zh: "导出用于离线开发",
            ja: "オフライン開発用にエクスポート",
            ko: "오프라인 개발을 위해 내보내기",
            fr: "Exporter pour le développement hors ligne",
            de: "Export für die Offline-Entwicklung",
            ru: "Экспорт для автономной разработки"
        },
        "exit" => {
            en: "Exit",
            zh: "退出",
            ja: "終了",
            ko: "종료",
            fr: "Quitter",
            de: "Beenden",
            ru: "Выход"
        },
        
        // 文件和依赖扫描
        "scanning_files" => {
            en: "Scanning Python files...",
            zh: "正在扫描Python文件...",
            ja: "Pythonファイルをスキャン中...",
            ko: "Python 파일 스캔 중...",
            fr: "Analyse des fichiers Python...",
            de: "Scanne Python-Dateien...",
            ru: "Сканирование файлов Python..."
        },
        "found_files" => {
            en: "Found {} Python files",
            zh: "找到{}个Python文件",
            ja: "{}個のPythonファイルが見つかりました",
            ko: "{}개의 Python 파일을 찾았습니다",
            fr: "{} fichiers Python trouvés",
            de: "{} Python-Dateien gefunden",
            ru: "Найдено {} файлов Python"
        },
        "found_dependencies" => {
            en: "Found {} dependencies",
            zh: "找到{}个依赖",
            ja: "{}個の依存関係が見つかりました",
            ko: "{}개의 종속성을 찾았습니다",
            fr: "{} dépendances trouvées",
            de: "{} Abhängigkeiten gefunden",
            ru: "Найдено {} зависимостей"
        },
        "no_dependencies" => {
            en: "No external dependencies found.",
            zh: "未找到外部依赖。",
            ja: "外部依存関係は見つかりませんでした。",
            ko: "외부 종속성을 찾을 수 없습니다.",
            fr: "Aucune dépendance externe trouvée.",
            de: "Keine externen Abhängigkeiten gefunden.",
            ru: "Внешние зависимости не найдены."
        },
        "external_dependencies" => {
            en: "Found the following external dependencies:",
            zh: "找到以下外部依赖：",
            ja: "次の外部依存関係が見つかりました：",
            ko: "다음 외부 종속성을 찾았습니다:",
            fr: "Les dépendances externes suivantes ont été trouvées :",
            de: "Die folgenden externen Abhängigkeiten wurden gefunden:",
            ru: "Найдены следующие внешние зависимости:"
        },
        
        // 本地开发
        "local_dev_title" => {
            en: "Local Development Setup",
            zh: "本地开发设置",
            ja: "ローカル開発セットアップ",
            ko: "로컬 개발 설정",
            fr: "Configuration du développement local",
            de: "Lokale Entwicklungseinrichtung",
            ru: "Настройка локальной разработки"
        },
        "no_python_files" => {
            en: "No Python files found!",
            zh: "未找到Python文件！",
            ja: "Pythonファイルが見つかりません！",
            ko: "Python 파일을 찾을 수 없습니다!",
            fr: "Aucun fichier Python trouvé !",
            de: "Keine Python-Dateien gefunden!",
            ru: "Файлы Python не найдены!"
        },
        "how_to_continue" => {
            en: "How to continue?",
            zh: "该如何继续？",
            ja: "どのように続けますか？",
            ko: "어떻게 계속하시겠습니까?",
            fr: "Comment continuer ?",
            de: "Wie möchten Sie fortfahren?",
            ru: "Как продолжить?"
        },
        "use_test_suite" => {
            en: "Use example files from test suite",
            zh: "使用测试套件中的示例文件",
            ja: "テストスイートからサンプルファイルを使用",
            ko: "테스트 스위트의 예제 파일 사용",
            fr: "Utiliser les fichiers exemple de la suite de test",
            de: "Beispieldateien aus der Testsuite verwenden",
            ru: "Использовать примеры файлов из тестового набора"
        },
        "specify_directory" => {
            en: "Manually specify Python files directory",
            zh: "手动指定Python文件目录",
            ja: "Pythonファイルディレクトリを手動で指定",
            ko: "Python 파일 디렉토리를 수동으로 지정",
            fr: "Spécifier manuellement le répertoire des fichiers Python",
            de: "Python-Dateiverzeichnis manuell angeben",
            ru: "Вручную указать каталог файлов Python"
        },
        "cancel" => {
            en: "Cancel operation",
            zh: "取消操作",
            ja: "操作をキャンセル",
            ko: "작업 취소",
            fr: "Annuler l'opération",
            de: "Vorgang abbrechen",
            ru: "Отменить операцию"
        },
        
        // 更多翻译...

        "requirements_created" => {
            en: "Created requirements.txt file in {}",
            zh: "创建了requirements.txt文件在 {}",
            ja: "{}にrequirements.txtファイルを作成しました",
            ko: "{}에 requirements.txt 파일을 생성했습니다",
            fr: "Fichier requirements.txt créé dans {}",
            de: "requirements.txt-Datei in {} erstellt",
            ru: "Файл requirements.txt создан в {}"
        },
        
        // 运行脚本
        "running_script" => {
            en: "Running Python script",
            zh: "运行Python脚本",
            ja: "Pythonスクリプトを実行中",
            ko: "Python 스크립트 실행 중",
            fr: "Exécution du script Python",
            de: "Python-Skript wird ausgeführt",
            ru: "Запуск скрипта Python"
        },
        "script" => {
            en: "Script: {}",
            zh: "脚本: {}",
            ja: "スクリプト: {}",
            ko: "스크립트: {}",
            fr: "Script : {}",
            de: "Skript: {}",
            ru: "Скрипт: {}"
        },
        
        // 使用提示
        "usage_tips" => {
            en: "PyWand Usage Tips:",
            zh: "PyWand 使用小贴士:",
            ja: "PyWand 使用方法のヒント:",
            ko: "PyWand 사용 팁:",
            fr: "Conseils d'utilisation de PyWand :",
            de: "PyWand Nutzungstipps:",
            ru: "Советы по использованию PyWand:"
        },

        // 添加下面这些新翻译
        "select_python_version" => {
            en: "Select Python version",
            zh: "选择Python版本",
            ja: "Pythonバージョンを選択",
            ko: "Python 버전 선택",
            fr: "Sélectionner la version Python",
            de: "Python-Version auswählen",
            ru: "Выберите версию Python"
        },
        "creating_venv" => {
            en: "Creating Python {} virtual environment...",
            zh: "正在创建Python {}虚拟环境...",
            ja: "Python {}仮想環境を作成しています...",
            ko: "Python {} 가상 환경 생성 중...",
            fr: "Création de l'environnement virtuel Python {}...",
            de: "Python {}-Virtualenv wird erstellt...",
            ru: "Создание виртуальной среды Python {}..."
        },
        "installing_dependencies" => {
            en: "Installing dependencies...",
            zh: "正在安装依赖...",
            ja: "依存関係をインストールしています...",
            ko: "종속성 설치 중...",
            fr: "Installation des dépendances...",
            de: "Abhängigkeiten werden installiert...",
            ru: "Установка зависимостей..."
        },
        "created_activation_scripts" => {
            en: "Created activation scripts",
            zh: "创建了激活脚本",
            ja: "アクティベーションスクリプトを作成しました",
            ko: "활성화 스크립트 생성됨",
            fr: "Scripts d'activation créés",
            de: "Aktivierungsskripte erstellt",
            ru: "Созданы скрипты активации"
        },
        "setup_complete" => {
            en: "Setup complete!",
            zh: "设置完成！",
            ja: "セットアップが完了しました！",
            ko: "설정 완료!",
            fr: "Configuration terminée !",
            de: "Einrichtung abgeschlossen!",
            ru: "Настройка завершена!"
        },
        "to_activate_venv" => {
            en: "To activate virtual environment, run:",
            zh: "要激活虚拟环境，请运行:",
            ja: "仮想環境を有効にするには、次を実行します：",
            ko: "가상 환경을 활성화하려면 실행하세요:",
            fr: "Pour activer l'environnement virtuel, exécutez :",
            de: "Um die virtuelle Umgebung zu aktivieren, führen Sie aus:",
            ru: "Чтобы активировать виртуальную среду, выполните:"
        },
        "exporting_offline" => {
            en: "Export for Offline Development",
            zh: "导出用于离线开发",
            ja: "オフライン開発用にエクスポート",
            ko: "오프라인 개발용 내보내기",
            fr: "Exporter pour le développement hors ligne",
            de: "Export für die Offline-Entwicklung",
            ru: "Экспорт для автономной разработки"
        },
        "select_os" => {
            en: "Select target operating system",
            zh: "选择目标操作系统",
            ja: "ターゲットオペレーティングシステムを選択",
            ko: "대상 운영 체제 선택",
            fr: "Sélectionner le système d'exploitation cible",
            de: "Ziel-Betriebssystem auswählen",
            ru: "Выберите целевую операционную систему"
        },
        "preparing_package" => {
            en: "Preparing package for {} and Python {}...",
            zh: "正在为{}和Python {}准备包...",
            ja: "{}とPython {}のパッケージを準備しています...",
            ko: "{}와 Python {}용 패키지 준비 중...",
            fr: "Préparation du package pour {} et Python {}...",
            de: "Paket für {} und Python {} wird vorbereitet...",
            ru: "Подготовка пакета для {} и Python {}..."
        },
        "files_copied" => {
            en: "Files copied successfully",
            zh: "文件复制成功",
            ja: "ファイルが正常にコピーされました",
            ko: "파일이 성공적으로 복사됨",
            fr: "Fichiers copiés avec succès",
            de: "Dateien erfolgreich kopiert",
            ru: "Файлы успешно скопированы"
        },
        "scripts_created" => {
            en: "Setup scripts created",
            zh: "创建了设置脚本",
            ja: "セットアップスクリプトが作成されました",
            ko: "설정 스크립트 생성됨",
            fr: "Scripts de configuration créés",
            de: "Setup-Skripte erstellt",
            ru: "Созданы скрипты настройки"
        },
        "readme_created" => {
            en: "README file created",
            zh: "创建了README文件",
            ja: "READMEファイルが作成されました",
            ko: "README 파일 생성됨",
            fr: "Fichier README créé",
            de: "README-Datei erstellt",
            ru: "Файл README создан"
        },
        "creating_archive" => {
            en: "Creating archive {}...",
            zh: "正在创建归档{}...",
            ja: "アーカイブ{}を作成しています...",
            ko: "아카이브 {} 생성 중...",
            fr: "Création de l'archive {}...",
            de: "Archiv {} wird erstellt...",
            ru: "Создание архива {}..."
        },
        "archive_created" => {
            en: "Archive created successfully",
            zh: "归档创建成功",
            ja: "アーカイブが正常に作成されました",
            ko: "아카이브가 성공적으로 생성됨",
            fr: "Archive créée avec succès",
            de: "Archiv erfolgreich erstellt",
            ru: "Архив успешно создан"
        },
        "export_complete" => {
            en: "Export completed successfully!",
            zh: "导出成功完成！",
            ja: "エクスポートが正常に完了しました！",
            ko: "내보내기가 성공적으로 완료되었습니다!",
            fr: "Exportation terminée avec succès !",
            de: "Export erfolgreich abgeschlossen!",
            ru: "Экспорт успешно завершен!"
        },
        "package_saved" => {
            en: "Package saved to: ./{}",
            zh: "包已保存到: ./{}",
            ja: "パッケージが./{}に保存されました",
            ko: "패키지가 ./{}에 저장됨",
            fr: "Package enregistré dans: ./{}",
            de: "Paket gespeichert unter: ./{}",
            ru: "Пакет сохранен в: ./{}"
        },
        "running_in_test" => {
            en: "Running in test mode with test suite",
            zh: "以测试模式运行，使用测试套件",
            ja: "テストスイートでテストモードで実行しています",
            ko: "테스트 스위트로 테스트 모드에서 실행 중",
            fr: "Exécution en mode test avec la suite de tests",
            de: "Ausführung im Testmodus mit Test-Suite",
            ru: "Запуск в тестовом режиме с использованием тестового набора"
        },
        "using_directory" => {
            en: "Using directory: {}",
            zh: "使用目录: {}",
            ja: "ディレクトリを使用: {}",
            ko: "디렉토리 사용: {}",
            fr: "Utilisation du répertoire : {}",
            de: "Verzeichnis wird verwendet: {}",
            ru: "Используется каталог: {}"
        },
        "running_local_dev" => {
            en: "Running local development workflow",
            zh: "直接执行本地开发流程",
            ja: "ローカル開発ワークフローを実行しています",
            ko: "로컬 개발 워크플로우 실행 중",
            fr: "Exécution du flux de développement local",
            de: "Lokaler Entwicklungsablauf wird ausgeführt",
            ru: "Выполнение рабочего процесса локальной разработки"
        },
        "generating_req" => {
            en: "Generating requirements.txt file",
            zh: "直接生成requirements.txt文件",
            ja: "requirements.txtファイルを生成しています",
            ko: "requirements.txt 파일 생성 중",
            fr: "Génération du fichier requirements.txt",
            de: "requirements.txt-Datei wird generiert",
            ru: "Создание файла requirements.txt"
        },
        "scanning_dir" => {
            en: "Scanning directory: {}",
            zh: "扫描目录: {}",
            ja: "ディレクトリをスキャン: {}",
            ko: "디렉토리 스캔: {}",
            fr: "Analyse du répertoire : {}",
            de: "Verzeichnis wird gescannt: {}",
            ru: "Сканирование каталога: {}"
        },
        "output_dir" => {
            en: "Output directory: {}",
            zh: "输出目录: {}",
            ja: "出力ディレクトリ: {}",
            ko: "출력 디렉토리: {}",
            fr: "Répertoire de sortie : {}",
            de: "Ausgabeverzeichnis: {}",
            ru: "Выходной каталог: {}"
        },
        "req_generated" => {
            en: "Requirements file generated!",
            zh: "要求文件已生成！",
            ja: "要件ファイルが生成されました！",
            ko: "요구 사항 파일이 생성되었습니다!",
            fr: "Fichier des exigences généré !",
            de: "Anforderungsdatei wurde generiert!",
            ru: "Файл требований создан!"
        },
        "no_command" => {
            en: "No command specified, using default workflow",
            zh: "无指定命令，使用默认流程",
            ja: "コマンドが指定されていません。デフォルトのワークフローを使用します",
            ko: "명령이 지정되지 않았습니다. 기본 워크플로우 사용",
            fr: "Aucune commande spécifiée, utilisation du flux par défaut",
            de: "Kein Befehl angegeben, Standardablauf wird verwendet",
            ru: "Команда не указана, используется рабочий процесс по умолчанию"
        },
        "scanning_current" => {
            en: "Scanning current directory",
            zh: "使用当前目录进行扫描",
            ja: "現在のディレクトリをスキャンしています",
            ko: "현재 디렉토리 스캔 중",
            fr: "Analyse du répertoire courant",
            de: "Aktuelles Verzeichnis wird gescannt",
            ru: "Сканирование текущего каталога"
        },
        
        // 语言设置
        "language_changed" => {
            en: "Language setting changed",
            zh: "语言设置已更改",
            ja: "言語設定が変更されました",
            ko: "언어 설정이 변경되었습니다",
            fr: "Paramètre de langue modifié",
            de: "Spracheinstellung geändert",
            ru: "Настройки языка изменены"
        },
        "unsupported_language" => {
            en: "Unsupported language code: {}. Using default language (system language)",
            zh: "不支持的语言代码: {}。使用默认语言(系统语言)",
            ja: "サポートされていない言語コード：{}。デフォルトの言語（システム言語）を使用します",
            ko: "지원되지 않는 언어 코드: {}. 기본 언어(시스템 언어) 사용",
            fr: "Code de langue non pris en charge : {}. Utilisation de la langue par défaut (langue du système)",
            de: "Nicht unterstützter Sprachcode: {}. Standardsprache (Systemsprache) wird verwendet",
            ru: "Неподдерживаемый код языка: {}. Используется язык по умолчанию (системный язык)"
        },
        "available_languages" => {
            en: "Available language codes",
            zh: "可用语言代码",
            ja: "利用可能な言語コード",
            ko: "사용 가능한 언어 코드",
            fr: "Codes de langue disponibles",
            de: "Verfügbare Sprachcodes",
            ru: "Доступные коды языков"
        },
        "scan_create_req" => {
            en: "Scan project and create requirements.txt",
            zh: "扫描项目并创建requirements.txt",
            ja: "プロジェクトをスキャンしてrequirements.txtを作成する",
            ko: "프로젝트를 스캔하고 requirements.txt 생성",
            fr: "Analyser le projet et créer requirements.txt",
            de: "Projekt scannen und requirements.txt erstellen",
            ru: "Сканировать проект и создать requirements.txt"
        },
        "setup_local_dev" => {
            en: "Set up local development environment",
            zh: "建立本地开发环境",
            ja: "ローカル開発環境を設定する",
            ko: "로컬 개발 환경 설정",
            fr: "Configurer l'environnement de développement local",
            de: "Lokale Entwicklungsumgebung einrichten",
            ru: "Настроить локальную среду разработки"
        },
        "export_to_other" => {
            en: "Export project to other platforms",
            zh: "导出项目到其他平台",
            ja: "プロジェクトを他のプラットフォームにエクスポートする",
            ko: "다른 플랫폼으로 프로젝트 내보내기",
            fr: "Exporter le projet vers d'autres plateformes",
            de: "Projekt auf andere Plattformen exportieren",
            ru: "Экспортировать проект на другие платформы"
        },
        "run_python_script" => {
            en: "Run Python script",
            zh: "运行Python脚本",
            ja: "Pythonスクリプトを実行する",
            ko: "Python 스크립트 실행",
            fr: "Exécuter un script Python",
            de: "Python-Skript ausführen",
            ru: "Запустить скрипт Python"
        },
        "execute_uv_command" => {
            en: "Execute UV command",
            zh: "执行UV命令",
            ja: "UVコマンドを実行する",
            ko: "UV 명령 실행",
            fr: "Exécuter la commande UV",
            de: "UV-Befehl ausführen",
            ru: "Выполнить команду UV"
        },
        "set_interface_language" => {
            en: "Set interface language",
            zh: "设置界面语言",
            ja: "インターフェース言語を設定する",
            ko: "인터페이스 언어 설정",
            fr: "Définir la langue de l'interface",
            de: "Oberflächensprache festlegen",
            ru: "Установить язык интерфейса"
        },
        "installing_packages" => {
            en: "Installing Python packages",
            zh: "安装Python包",
            ja: "Pythonパッケージをインストールする",
            ko: "Python 패키지 설치",
            fr: "Installation des paquets Python",
            de: "Python-Pakete installieren",
            ru: "Установка пакетов Python"
        },
        "packages_installed" => {
            en: "Packages installed successfully",
            zh: "包安装成功",
            ja: "パッケージが正常にインストールされました",
            ko: "패키지가 성공적으로 설치됨",
            fr: "Paquets installés avec succès",
            de: "Pakete erfolgreich installiert",
            ru: "Пакеты успешно установлены"
        },
        "packages_install_failed" => {
            en: "Package installation failed",
            zh: "包安装失败",
            ja: "パッケージのインストールに失敗しました",
            ko: "패키지 설치 실패",
            fr: "L'installation du paquet a échoué",
            de: "Paketinstallation fehlgeschlagen",
            ru: "Установка пакета не удалась"
        },
        "install_python_packages" => {
            en: "Install Python packages",
            zh: "安装Python包",
            ja: "Pythonパッケージをインストールする",
            ko: "Python 패키지 설치",
            fr: "Installer des paquets Python",
            de: "Python-Pakete installieren",
            ru: "Установить пакеты Python"
        }
    };
}

pub struct I18n {
    pub language: Language,
}

impl I18n {
    pub fn new() -> Self {
        I18n {
            language: Language::default(),
        }
    }
    
    pub fn with_language(language: Language) -> Self {
        I18n {
            language,
        }
    }
    
    pub fn get<'a>(&self, key: &'a str) -> &'a str {
        TRANSLATIONS
            .get(key)
            .and_then(|translations| translations.get(&self.language))
            .copied()
            .unwrap_or_else(move || {
                // 回退到英语，如果找不到就返回键名
                TRANSLATIONS
                    .get(key)
                    .and_then(|translations| translations.get(&Language::English))
                    .copied()
                    .unwrap_or(key)
            })
    }
    
    pub fn get_formatted(&self, key: &str, args: &[&str]) -> String {
        let template = self.get(key);
        args.iter().enumerate().fold(template.to_string(), |acc, (i, arg)| {
            acc.replace(&format!("{{{}}}", i), arg)
        })
    }
    
    pub fn current_language(&self) -> Language {
        self.language
    }
    
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
} 