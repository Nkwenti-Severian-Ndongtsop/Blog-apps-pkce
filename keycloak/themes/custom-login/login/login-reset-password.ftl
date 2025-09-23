<#import "template.ftl" as layout>
<@layout.registrationLayout displayInfo=true displayMessage=!messagesPerField.existsError('username'); section>
    <#if section = "header">
        ${msg("emailForgotTitle")}
    <#elseif section = "form">
        <div id="kc-form">
            <div id="kc-form-wrapper">
                <div class="login-card">
                    <div class="login-header">
                        <h1>Reset Password</h1>
                        <p>Enter your email to receive reset instructions</p>
                    </div>
                    <form id="kc-reset-password-form" action="${url.loginAction}" method="post">
                        <div class="form-group">
                            <label for="username" class="form-label">${msg("usernameOrEmail")}</label>
                            <input type="text" id="username" name="username" class="form-input" autofocus value="${(auth.attemptedUsername!'')}" aria-invalid="<#if messagesPerField.existsError('username')>true</#if>"/>
                            <#if messagesPerField.existsError('username')>
                                <span id="input-error-username" class="${properties.kcInputErrorMessageClass!}" aria-live="polite">
                                    ${kcSanitize(messagesPerField.get('username'))?no_esc}
                                </span>
                            </#if>
                        </div>
                        <div class="form-buttons">
                            <input class="login-btn" type="submit" value="Send Reset Link"/>
                        </div>
                        <div style="text-align: center; margin-top: 20px;">
                            <a href="${url.loginUrl}">${kcSanitize(msg("backToLogin"))?no_esc}</a>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    <#elseif section = "info" >
        <div class="login-card" style="margin-top: 20px;">
            ${msg("emailInstruction")}
        </div>
    </#if>
</@layout.registrationLayout>
