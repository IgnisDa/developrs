import { Resolver, Query, Args } from '@nestjs/graphql';
import { UseGuards } from '@nestjs/common';
import { AuthService } from './auth.service';
import { GqlAuthGuard } from './guards/gql-auth.guard';
import { LoginUserInput } from './dto/login-user.input';
import { LoginResultUnion } from './dto/login-user.result';
import { CurrentUser } from './current-user.decorator';
import { UserDto } from 'src/user/dto/user.dto';
import { RefreshTokenResultUnion } from './dto/refresh-token.result';

@Resolver()
export class AuthResolver {
  constructor(private readonly authService: AuthService) {}

  @Query(() => LoginResultUnion, {
    description: 'Query to login using a username and password',
  })
  async loginUser(@Args('LoginUserInput') loginUserInput: LoginUserInput) {
    const res = await this.authService.validateUserByPassword(loginUserInput);
    if (res)
      return {
        __typename: 'LoginResult',
        ...res,
      };
    return {
      __typename: 'LoginError',
      message: 'Could not login with the provided credentials',
    };
  }

  @UseGuards(GqlAuthGuard)
  @Query(() => RefreshTokenResultUnion, {
    description: 'Get the refresh token using the authorization request header',
  })
  async refreshToken(@CurrentUser() user: UserDto) {
    const token = this.authService.createJwt(user);
    if (token)
      return {
        __typename: 'RefreshToken',
        token: token,
      };
    return {
      __typename: 'RefreshTokenError',
      message: 'Could not log in with the provided credentials',
    };
  }
}
