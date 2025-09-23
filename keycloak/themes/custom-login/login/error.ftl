<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=false; section>
    <#if section = "header">
        <#-- No header title -->
    <#elseif section = "form">
        <div id="kc-form">
            <div id="kc-form-wrapper">
                <div class="login-card">
                    <div class="login-header">
                        <h1>Something went wrong</h1>
                        <p>We encountered an unexpected error</p>
                    </div>
                    
                    <#if message?has_content && message.summary??>
                        <div class="error-message">
                            <p>${kcSanitize(message.summary)?no_esc}</p>
                        </div>
                    </#if>
                    
                    <div class="form-buttons">
                        <#if client?? && client.baseUrl?has_content>
                            <a href="${client.baseUrl}" class="login-btn">Back to Application</a>
                        <#else>
                            <a href="${url.loginUrl}" class="login-btn">Back to Login</a>
                        </#if>
                    </div>
                </div>
            </div>
        </div>
    </#if>
</@layout.registrationLayout>
