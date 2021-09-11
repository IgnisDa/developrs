import { createUnionType, ObjectType } from '@nestjs/graphql';
import { UserDto } from 'src/user/dto/user.dto';
import { APIError } from '../../core/dto/api-error.interface';

@ObjectType({ description: 'The type returned on successful login' })
export class LoginResult {
  /* The user this login result is associated with */
  user: UserDto;

  /* The JWT token to be used for authentication */
  token: string;
}

@ObjectType({
  description: 'The type returned for the errors when login is unsuccessful',
  implements: () => APIError,
})
export class LoginError extends APIError {
  /* The error message */
  message: string;
}

export const LoginResultUnion = createUnionType({
  name: 'LoginResultUnion',
  types: () => [LoginResult, LoginError],
  description: 'Result type returned as the result when someone tries to login',
});
