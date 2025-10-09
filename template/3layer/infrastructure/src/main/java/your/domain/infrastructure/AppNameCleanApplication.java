package {{yourDomain}}.infrastructure;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.autoconfigure.domain.EntityScan;
import org.springframework.context.annotation.ComponentScan;
import org.springframework.data.jpa.repository.config.EnableJpaRepositories;

@SpringBootApplication
@ComponentScan(basePackages = {
        "{{yourDomain}}.application",
        "{{yourDomain}}.core",
        "{{yourDomain}}.infrastructure"
})
@EnableJpaRepositories(basePackages = "{{yourDomain}}.infrastructure.adapters.repositories.jpa")
@EntityScan(basePackages = "{{yourDomain}}.infrastructure.adapters.repositories.entities")
public class {{AppNameClean}}Application {

	public static void main(String[] args) {
		SpringApplication.run({{AppNameClean}}Application.class, args);
	}

}
