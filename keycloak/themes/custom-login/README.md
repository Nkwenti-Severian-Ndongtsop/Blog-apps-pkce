# Custom Keycloak Login Theme

This is a custom Keycloak theme that provides a clean, minimal login interface matching the provided design mockup. The theme uses FTL (FreeMarker Template Language) files and includes embedded CSS for styling.

## Theme Features

- **Clean Design**: Minimal white card on light gray background
- **Responsive**: Works on desktop and mobile devices
- **Complete**: Includes login, registration, and password reset pages
- **Accessible**: Proper form labels and ARIA attributes
- **Modern Styling**: Clean typography and smooth transitions

## Theme Structure

```
custom-login/
├── theme.properties          # Theme configuration
├── login/
│   ├── template.ftl         # Base template with embedded CSS
│   ├── login.ftl           # Main login page
│   ├── register.ftl        # Registration page
│   └── login-reset-password.ftl  # Password reset page
└── README.md               # This file
```

## Installation Instructions

### Method 1: Copy to Keycloak Themes Directory

1. **Copy the theme folder** to your Keycloak themes directory:
   ```bash
   cp -r /home/nkwentiseverian/projects/keycloak-themes/Blog-apps/keycloak/themes/custom-login \
         /path/to/your/keycloak/themes/
   ```

2. **Restart Keycloak** to load the new theme.

3. **Enable the theme in Keycloak Admin Console**:
   - Log into Keycloak Admin Console
   - Go to your realm (e.g., "master" or your blog app realm)
   - Navigate to **Realm Settings** → **Themes**
   - Set **Login Theme** to "Custom Login Theme"
   - Click **Save**

### Method 2: Docker Volume Mount (if using Docker)

If you're running Keycloak in Docker, you can mount the theme directory:

```yaml
# In your docker-compose.yml
services:
  keycloak:
    image: quay.io/keycloak/keycloak:26.3.3
    volumes:
      - ./keycloak/themes/custom-login:/opt/keycloak/themes/custom-login
    # ... other configuration
```

### Method 3: For Development (Hot Reload)

For development with hot reload:

1. Set the `KC_SPI_THEME_STATIC_MAX_AGE` environment variable to `-1`
2. Set `KC_SPI_THEME_CACHE_THEMES` to `false`
3. Mount or copy the theme to the Keycloak themes directory

## Configuration

The theme inherits from Keycloak's base theme and includes:

- **Parent Theme**: `keycloak` (inherits base functionality)
- **Supported Locales**: English (`en`)
- **Display Name**: "Custom Login Theme"

## Customization

### Colors and Styling

The main styles are embedded in `login/template.ftl`. Key CSS variables you can modify:

```css
/* Background color */
background-color: #e8e8e8;

/* Card background */
background: white;

/* Button color */
background-color: #333;

/* Input focus color */
border-color: #007bff;
```

### Adding Custom CSS

To add additional styles, you can:

1. Create a `resources/css/` directory in the theme
2. Add your CSS files there
3. Reference them in `theme.properties`:
   ```properties
   styles=css/your-custom-styles.css
   ```

### Adding Custom JavaScript

To add JavaScript functionality:

1. Create a `resources/js/` directory
2. Add your JS files there
3. Reference them in `theme.properties`:
   ```properties
   scripts=js/your-custom-script.js
   ```

## Testing

1. **Start your Keycloak server**
2. **Navigate to the login page** of your blog app
3. **Verify the theme** displays correctly with:
   - Clean white card design
   - Proper form fields (USERNAME, PASSWORD)
   - Dark login button
   - Remember me checkbox
   - Forgot password link

## Troubleshooting

### Theme Not Appearing

1. **Check theme directory**: Ensure the theme is in the correct Keycloak themes directory
2. **Restart Keycloak**: Theme changes require a restart
3. **Clear browser cache**: Force refresh the login page
4. **Check logs**: Look for theme-related errors in Keycloak logs

### Styling Issues

1. **Check CSS syntax**: Ensure all CSS in `template.ftl` is valid
2. **Browser developer tools**: Inspect elements to debug styling
3. **FTL syntax**: Verify FreeMarker template syntax is correct

### Form Functionality

1. **Check FTL variables**: Ensure all Keycloak variables are properly referenced
2. **Test all flows**: Login, registration, password reset
3. **Validate messages**: Check error and success message display

## Support

For issues with this theme:

1. Check Keycloak documentation for theme development
2. Verify FreeMarker template syntax
3. Test with different browsers and devices
4. Check Keycloak server logs for errors

## Version Compatibility

This theme is designed for:
- **Keycloak Version**: 26.3.3 (compatible with other recent versions)
- **Template Engine**: FreeMarker (FTL)
- **Browser Support**: Modern browsers (Chrome, Firefox, Safari, Edge)
