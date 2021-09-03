import { createUnionType, ObjectType } from '@nestjs/graphql';
import { APIError } from './api-error.interface';

@ObjectType({ description: 'The type that contains the return token' })
export class RefreshToken {
  /* The refresh token */
  token: string;
}

@ObjectType({
  description: 'The error returned when trying to refresh the token',
  implements: () => APIError,
})
export class RefreshTokenError extends APIError {
  /* The error message when getting a refresh token */
  message: string;
}

export const RefreshTokenResultUnion = createUnionType({
  name: 'RefreshTokenResultUnion',
  types: () => [RefreshTokenError, RefreshToken],
  description:
    'Result type returned as the result when a client tries to create a refresh token',
});
