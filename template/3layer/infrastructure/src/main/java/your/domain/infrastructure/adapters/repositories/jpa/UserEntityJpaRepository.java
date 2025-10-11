package {{yourDomain}}.infrastructure.adapters.repositories.jpa;

import {{yourDomain}}.infrastructure.adapters.repositories.entities.{{UserEntity}}Entity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface {{UserEntity}}JpaRepository extends JpaRepository<{{UserEntity}}Entity, Long> {
    Optional<{{UserEntity}}Entity> findByEmail(String email);
}