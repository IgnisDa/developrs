import { InputType } from '@nestjs/graphql';

@InputType({ description: 'The input type used while logging in' })
export class LoginUserInput {
  /* The username of the user logging in */
  username: string;

  /* The password of the user logging in */
  password: string;
}
