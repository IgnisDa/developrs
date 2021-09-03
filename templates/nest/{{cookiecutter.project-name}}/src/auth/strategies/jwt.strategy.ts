import { Injectable } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { Strategy, ExtractJwt } from 'passport-jwt';
import { JwtPayload } from '../interfaces/jwt-payload.interface';
import { AuthenticationError } from 'apollo-server-core';
import { AuthService } from '../auth.service';

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  constructor(private authService: AuthService) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      secretOrKey: `${process.env.JWT_SECRET}`,
    });
  }

  async validate(payload: JwtPayload) {
    // This is called to validate the user in the token exists
    const user = await this.authService.validateJwtPayload(payload);

    if (!user) {
      throw new AuthenticationError(
        'Could not log in with the provided credentials',
      );
    }

    return user;
  }
}
