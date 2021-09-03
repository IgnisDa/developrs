import { Module } from '@nestjs/common';
import { PrismaService } from 'src/prisma.service';
import { UserResolver } from './user.resolver';
import { UserService } from './user.service';

@Module({
  providers: [UserResolver, UserService, PrismaService],
  exports: [UserService],
})
export class UserModule {}
