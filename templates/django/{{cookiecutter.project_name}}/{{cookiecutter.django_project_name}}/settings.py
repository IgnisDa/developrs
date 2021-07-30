import os
from pathlib import Path

# Build paths inside the project like this: BASE_DIR / 'subdir'.
BASE_DIR = Path(__file__).resolve().parent.parent

# SECURITY WARNING: keep the secret key used in production secret!
SECRET_KEY = os.getenv("DJANGO_SECRET_KEY", "dummy")

# SECURITY WARNING: don't run with debug turned on in production!
DEBUG = os.environ.get("DJANGO_DEBUG", False) == "1"

ALLOWED_HOSTS = []

# Application definition

INSTALLED_APPS = [
    "django.contrib.admin",
    "django.contrib.auth",
    "django.contrib.contenttypes",
    "django.contrib.sessions",
    "django.contrib.messages",
    "django.contrib.staticfiles",
    # user installed apps
    "{{cookiecutter.django_user_model_app}}.apps.{{cookiecutter.django_user_model_app|capitalize}}Config",
    # django addons
    "corsheaders",
]

MIDDLEWARE = [
    "django.middleware.security.SecurityMiddleware",
    "django.contrib.sessions.middleware.SessionMiddleware",
    "corsheaders.middleware.CorsMiddleware",
    "django.middleware.common.CommonMiddleware",
    "django.middleware.csrf.CsrfViewMiddleware",
    "django.contrib.auth.middleware.AuthenticationMiddleware",
    "django.contrib.messages.middleware.MessageMiddleware",
    "django.middleware.clickjacking.XFrameOptionsMiddleware",
]

ROOT_URLCONF = "{{cookiecutter.django_project_name}}.urls"

TEMPLATES = [
    {
        "BACKEND": "django.template.backends.django.DjangoTemplates",
        "DIRS": [],
        "APP_DIRS": True,
        "OPTIONS": {
            "context_processors": [
                "django.template.context_processors.debug",
                "django.template.context_processors.request",
                "django.contrib.auth.context_processors.auth",
                "django.contrib.messages.context_processors.messages",
            ],
        },
    },
]

WSGI_APPLICATION = "{{cookiecutter.django_project_name}}.wsgi.application"

# Password validation
# https://docs.djangoproject.com/en/3.1/ref/settings/#auth-password-validators

AUTH_PASSWORD_VALIDATORS = [
    {
        "NAME": "django.contrib.auth.password_validation.UserAttributeSimilarityValidator",  # noqa: E501
    },
    {
        "NAME": "django.contrib.auth.password_validation.MinimumLengthValidator",
    },
    {
        "NAME": "django.contrib.auth.password_validation.CommonPasswordValidator",
    },
    {
        "NAME": "django.contrib.auth.password_validation.NumericPasswordValidator",
    },
]


# Internationalization
# https://docs.djangoproject.com/en/3.1/topics/i18n/

LANGUAGE_CODE = "en-us"

TIME_ZONE = "UTC"

USE_I18N = True

USE_L10N = True

USE_TZ = True


# Static files (CSS, JavaScript, Images)
# https://docs.djangoproject.com/en/3.1/howto/static-files/

STATIC_URL = "/static/"

# Settings added later

PASSWORD_HASHERS = [
    "django.contrib.auth.hashers.PBKDF2PasswordHasher",
    "django.contrib.auth.hashers.PBKDF2SHA1PasswordHasher",
]
CORS_ORIGIN_WHITELIST = ("http://127.0.0.1:3000", "http://localhost:3000")
TIME_ZONE = "Asia/Kolkata"
DATABASES = {
    "default": {
        "ENGINE": os.environ.get("DJANGO_DATABASE_ENGINE", "django.db.backends.sqlite3"),
        "NAME": os.environ.get("DJANGO_DATABASE_NAME", BASE_DIR / "db.sqlite3"),
        "USER": os.environ.get("DJANGO_DATABASE_USER", "admin-user"),
        "PASSWORD": os.environ.get("DJANGO_DATABASE_PASSWORD", "admin-password"),
        "HOST": os.environ.get("DJANGO_DATABASE_HOST", "localhost"),
        "PORT": "5432",
    }
}
AUTH_USER_MODEL = "{{cookiecutter.django_user_model_app}}.CustomUser"
STATIC_ROOT = BASE_DIR / "static"
MEDIA_ROOT = "media"
# production only settings
if not DEBUG:
    PASSWORD_HASHERS.insert(
        0,
        "django.contrib.auth.hashers.Argon2PasswordHasher",
    )
# development only settings
if DEBUG:
    ALLOWED_HOSTS += ["*"]
    CORS_ALLOW_ALL_ORIGINS = True
    INSTALLED_APPS.append("django_extensions")
    SHELL_PLUS = "ipython"
