<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=!messagesPerField.existsError('username','password') displayInfo=realm.password && realm.registrationAllowed && !registrationDisabled??; section>
    <#if section = "header">
        <#-- No header title -->
    <#elseif section = "form">
    <div id="kc-form">
      <div id="kc-form-wrapper">
        <div class="login-card">
          <div class="login-header">
            <h1>Sign In</h1>
            <p>Access your account securely</p>
          </div>
          
          <#if realm.password>l
            <form id="kc-form-login" onsubmit="login.disabled = true; return true;" action="${url.loginAction}" method="post">
              <div class="form-group">
                <label for="username" class="form-label">USERNAME</label>
                <#if usernameEditDisabled??>
                  <input tabindex="1" id="username" class="form-input" name="username" value="${(login.username!'')}" type="text" disabled />
                <#else>
                  <input tabindex="1" id="username" class="form-input" name="username" value="${(login.username!'')}"  type="text" autofocus autocomplete="off"
                         aria-invalid="<#if messagesPerField.existsError('username','password')>true</#if>"
                  />
                </#if>
              </div>

              <div class="form-group">
                <label for="password" class="form-label">PASSWORD</label>
                <input tabindex="2" id="password" class="form-input" name="password" type="password" autocomplete="off"
                       aria-invalid="<#if messagesPerField.existsError('username','password')>true</#if>"
                />
              </div>

              <div class="form-options">
                <#if realm.rememberMe && !usernameEditDisabled??>
                  <div class="checkbox">
                    <label>
                      <#if login.rememberMe??>
                        <input tabindex="3" id="rememberMe" name="rememberMe" type="checkbox" checked> ${msg("rememberMe")}
                      <#else>
                        <input tabindex="3" id="rememberMe" name="rememberMe" type="checkbox"> ${msg("rememberMe")}
                      </#if>
                    </label>
                  </div>
                </#if>
                
                <#if realm.resetPasswordAllowed>
                  <div class="forgot-password">
                    <a tabindex="5" href="${url.loginResetCredentialsUrl}">${msg("doForgotPassword")}</a>
                  </div>
                </#if>
              </div>

              <div id="kc-form-buttons" class="form-buttons">
                <input type="hidden" id="id-hidden-input" name="credentialId" <#if auth.selectedCredential?has_content>value="${auth.selectedCredential}"</#if>/>
                <input tabindex="4" class="login-btn" name="login" id="kc-login" type="submit" value="Login"/>
              </div>
              
              <#if realm.password && realm.registrationAllowed && !registrationDisabled??>
                <div class="login-register-link">
                    <span>Don't have an account? <a tabindex="6" href="${url.registrationUrl}">Register</a></span>
                </div>
              </#if>
            </form>
          </#if>
        </div>
      </div>
    </div>
    <#elseif section = "info" >
    </#if>

</@layout.registrationLayout>
