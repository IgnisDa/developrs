import { Injectable } from '@nestjs/common';
import { CreateUserInput } from './dto/create-user.input';
import { checkPropertiesExists, mergeObjects, validateObject } from 'src/utils';
import { CreateUserError } from './dto/create-user.result';
import { plainToClass } from 'class-transformer';
import { PrismaService } from 'src/prisma.service';
import { getPasswordDigest } from './utils';

@Injectable()
export class UserService {
  constructor(private readonly prisma: PrismaService) {
    prisma.$use(async (params, next) => {
      if (
        params.model === 'User' &&
        (params.action === 'update' || params.action === "create")
      ) {
        if (params.args.data.password) {
          params.args.data.password = await getPasswordDigest(
            params.args.data.password,
          );
        }
      }
      return next(params);
    });
  }

  async createUser(createUserInput: CreateUserInput) {
    let errors = new CreateUserError();
    const validationErrors = plainToClass(
      CreateUserError,
      await validateObject(createUserInput, CreateUserInput),
    );
    const resp = { status: false, resp: null };
    const usernameExists = await this.prisma.user.count({
      where: {
        username: { equals: createUserInput.username },
      },
    });
    if (usernameExists !== 0) {
      errors.usernameErrors.push('this user already exists');
    }
    const emailExists = await this.prisma.user.count({
      where: {
        email: { equals: createUserInput.email },
      },
    });
    if (emailExists !== 0) {
      errors.emailErrors.push('this email already exists');
    }
    // TODO
    // @ts-expect-error ts-migrate(2339) FIXME: Property 'usernameErrors' does not exist on type '{}'.
    errors = mergeObjects(errors, ...validationErrors);
    if (!checkPropertiesExists(Object(errors))) {
      resp.resp = errors;
      return resp;
    }
    resp.status = true;
    resp.resp = await this.prisma.user.create({ data: createUserInput });
    return resp;
  }

  async getUserByUsername(username: string) {
    return await this.prisma.user.findUnique({ where: { username } });
  }
}
