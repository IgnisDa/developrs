from django.contrib.auth.models import AbstractUser
from django.db import models
from django.utils.translation import gettext_lazy as _

from . import managers


class CustomUser(AbstractUser):
    username = models.CharField(max_length=100, help_text=_("The username of the user."))
    email = models.EmailField(help_text=_("The email address of the user."), unique=True)

    USERNAME_FIELD = "email"
    REQUIRED_FIELDS = []

    objects = managers.CustomUserManager()

    class Meta:
        ordering = ("id",)

    def __str__(self):
        return self.email
