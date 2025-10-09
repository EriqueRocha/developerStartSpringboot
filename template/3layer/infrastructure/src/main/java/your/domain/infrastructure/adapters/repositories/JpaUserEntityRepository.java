package {{yourDomain}}.infrastructure.adapters.repositories;

import {{yourDomain}}.application.ports.repositories.{{UserEntity}}Repository;
import {{yourDomain}}.core.domain.entities.{{UserEntity}};
import {{yourDomain}}.infrastructure.adapters.repositories.entities.{{UserEntity}}Entity;
import {{yourDomain}}.infrastructure.adapters.repositories.jpa.{{UserEntity}}JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public class Jpa{{UserEntity}}Repository implements {{UserEntity}}Repository {
    private final {{UserEntity}}JpaRepository jpaRepository;

    public Jpa{{UserEntity}}Repository({{UserEntity}}JpaRepository jpaRepository) {
        this.jpaRepository = jpaRepository;
    }

    @Override
    public Optional<{{UserEntity}}> findByEmail(String email) {
        return jpaRepository.findByEmail(email)
                .map(this::toDomain);
    }

    @Override
    public {{UserEntity}} save({{UserEntity}} {{userEntity}}) {
        {{AuthorEntity}} entity = toEntity({{userEntity}});
        {{AuthorEntity}} savedEntity = jpaRepository.save(entity);
        return toDomain(savedEntity);
    }

    @Override
    public Optional<{{UserEntity}}> findById(Long id) {
        return jpaRepository.findById(id)
                .map(this::toDomain);
    }

    private {{UserEntity}} toDomain({{AuthorEntity}} entity) {
        return new {{UserEntity}}(
                entity.getId(),
                entity.getEmail(),
                entity.getPassword(),
                entity.getName(),
                entity.getCreatedAt(),
                entity.getUpdatedAt()
        );
    }

    private {{AuthorEntity}} toEntity({{UserEntity}} domain) {
        {{AuthorEntity}} entity = new {{AuthorEntity}}();
        entity.setId(domain.getId());
        entity.setEmail(domain.getEmail());
        entity.setPassword(domain.getPassword());
        entity.setName(domain.getName());
        entity.setCreatedAt(domain.getCreatedAt());
        entity.setUpdatedAt(domain.getUpdatedAt());
        return entity;
    }
}