import * as argon2 from 'argon2';

export async function getPasswordDigest(password: string) {
  const passwordDigest = await argon2.hash(password, {
    memoryCost: 2 ** 20,
  });
  return passwordDigest;
}

export async function checkPassword(password: string, passwordDigest: string) {
  return await argon2.verify(passwordDigest, password);
}
