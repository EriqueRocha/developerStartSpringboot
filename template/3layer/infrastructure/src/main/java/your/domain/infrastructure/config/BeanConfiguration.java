package {{yourDomain}}.infrastructure.config;

import {{yourDomain}}.application.ports.repositories.AdminRepository;
import {{yourDomain}}.application.ports.repositories.{{UserEntity}}Repository;
import {{yourDomain}}.application.ports.services.PasswordService;
import {{yourDomain}}.application.ports.services.TokenService;
import {{yourDomain}}.application.usecases.AuthenticateAdminUseCase;
import {{yourDomain}}.application.usecases.Authenticate{{UserEntity}}UseCase;
import {{yourDomain}}.application.usecases.Create{{UserEntity}}UseCase;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class BeanConfiguration {

    @Bean
    public Authenticate{{UserEntity}}UseCase authenticate{{UserEntity}}UseCase(
            {{UserEntity}}Repository authorRepository,
            PasswordService passwordService,
            TokenService tokenService) {
        return new Authenticate{{UserEntity}}UseCase(authorRepository, passwordService, tokenService);
    }

    @Bean
    public AuthenticateAdminUseCase authenticateAdminUseCase(
            AdminRepository adminRepository,
            PasswordService passwordService,
            TokenService tokenService) {
        return new AuthenticateAdminUseCase(adminRepository, passwordService, tokenService);
    }

    @Bean
    public Create{{UserEntity}}UseCase create{{UserEntity}}UseCase(
            {{UserEntity}}Repository authorRepository,
            PasswordService passwordService) {
        return new Create{{UserEntity}}UseCase(authorRepository, passwordService);
    }
}
