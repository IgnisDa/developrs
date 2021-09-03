import { Field, ID, ObjectType } from '@nestjs/graphql';

@ObjectType({ description: 'Details about the user object' })
export class UserDto {
  @Field(() => ID, { description: 'The primary key of the user' })
  id: string;

  /* The name of the user */
  username: string;

  /* The email of the user */
  email: string;
}
