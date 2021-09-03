import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import { GraphQLModule } from '@nestjs/graphql';
import { AuthModule } from './auth/auth.module';
import { UserModule } from './user/user.module';
import { join } from 'path';
import * as Joi from 'joi';

@Module({
  imports: [
    GraphQLModule.forRoot({
      autoSchemaFile: join(process.cwd(), 'src', 'schema.graphql'),
      playground: process.env.NODE_ENV !== "production"
    }),
    ConfigModule.forRoot({
      validationSchema: Joi.object({
        JWT_SECRET: Joi.string().required(),
        JWT_EXPIRES_IN: Joi.string().required(),
        DATABASE_URL: Joi.string().required(),
      }),
    }),
    AuthModule,
    UserModule
  ],
  controllers: [],
  providers: [],
})
export class AppModule { }
