package {{yourDomain}}.application.ports.repositories;

import {{yourDomain}}.core.domain.entities.{{UserEntity}};

import java.util.Optional;

public interface {{UserEntity}}Repository {
    Optional<{{UserEntity}}> findByEmail(String email);
    {{UserEntity}} save({{UserEntity}} {{userEntity}});
    Optional<{{UserEntity}}> findById(Long id);
}