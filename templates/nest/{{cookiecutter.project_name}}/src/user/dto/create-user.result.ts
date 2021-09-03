import { createUnionType, ObjectType } from '@nestjs/graphql';
import { UserDto } from './user.dto';

@ObjectType({
  description: 'Type returned for the errors when a new user is created',
})
export class CreateUserError {
  /* The error associated with username */
  usernameErrors?: string[] | null = [];

  /* The error associated with email */
  emailErrors?: string[] | null = [];

  /* The error associated with password */
  passwordErrors?: string[] | null = [];
}

export const CreateUserResultUnion = createUnionType({
  name: 'CreateUserResultUnion',
  types: () => [CreateUserError, UserDto],
  description: 'Result type returned as the result when new user is created',
});
