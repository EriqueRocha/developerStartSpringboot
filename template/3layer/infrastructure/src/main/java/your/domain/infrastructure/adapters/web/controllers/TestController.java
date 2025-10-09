package {{yourDomain}}.infrastructure.adapters.web.controllers;

import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api")
public class TestController {

    @GetMapping("/admin/dashboard")
    public String adminDashboard(Authentication authentication) {
        return "Welcome to Admin Dashboard, " + authentication.getName() + "!";
    }

    @GetMapping("/{{userEntity}}/profile")
    public String authorProfile(Authentication authentication) {
        return "Welcome to Author Profile, " + authentication.getName() + "!";
    }

    @GetMapping("/public")
    public String publicEndpoint() {
        return "This is a public endpoint!";
    }
}
