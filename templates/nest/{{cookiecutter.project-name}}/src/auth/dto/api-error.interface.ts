import { InterfaceType } from '@nestjs/graphql';

@InterfaceType({
  description:
    'The standard interface that contains the error message when something goes wrong',
})
export abstract class APIError {
  /* The error message */
  message: string;
}
