package {{yourDomain}}.application.usecases;

import {{yourDomain}}.application.ports.repositories.{{UserEntity}}Repository;
import {{yourDomain}}.application.ports.services.PasswordService;
import {{yourDomain}}.core.domain.entities.UserEntity;

import java.util.Optional;

public class Create{{UserEntity}}UseCase {
    private final {{UserEntity}}Repository {{userEntity}}Repository;
    private final PasswordService passwordService;

    public Create{{UserEntity}}UseCase({{UserEntity}}Repository {{userEntity}}Repository, PasswordService passwordService) {
        this.{{userEntity}}Repository = {{userEntity}}Repository;
        this.passwordService = passwordService;
    }

    public Optional<{{UserEntity}}> execute(String email, String password, String name) {
        Optional<{{UserEntity}}> existing{{UserEntity}} = {{userEntity}}Repository.findByEmail(email);
        if (existing{{UserEntity}}.isPresent()) {
            return Optional.empty();
        }

        String encodedPassword = passwordService.encode(password);
        {{UserEntity}} new{{UserEntity}} = new {{UserEntity}}(email, encodedPassword, name);
        {{UserEntity}} saved{{UserEntity}} = {{userEntity}}Repository.save(new{{UserEntity}});

        return Optional.of(saved{{UserEntity}});
    }
}
