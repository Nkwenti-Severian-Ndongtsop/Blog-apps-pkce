<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=false; section>
    <#if section = "header">
        <#-- No header title -->
    <#elseif section = "form">
        <div id="kc-form">
            <div id="kc-form-wrapper">
                <div class="login-card">
                    <div class="login-header">
                        <h1>Information</h1>
                        <p>Please review the information below</p>
                    </div>
                    
                    <#if messageHeader??>
                        <div class="info-header">
                            <h2>${messageHeader}</h2>
                        </div>
                    </#if>
                    
                    <#if message?has_content && message.summary??>
                        <div class="info-message">
                            <p>${kcSanitize(message.summary)?no_esc}</p>
                        </div>
                    </#if>
                    
                    <#if requiredActions??>
                        <div class="required-actions">
                            <#list requiredActions as reqActionItem>
                                <div class="required-action-item">
                                    ${msg("requiredAction.${reqActionItem}")}
                                </div>
                            </#list>
                        </div>
                    </#if>
                    
                    <div class="form-buttons">
                        <#if skipLink??>
                            <a href="${skipLink}" class="login-btn">Continue</a>
                        <#elseif pageRedirectUri??>
                            <a href="${pageRedirectUri}" class="login-btn">Continue</a>
                        <#elseif actionUri??>
                            <a href="${actionUri}" class="login-btn">Continue</a>
                        <#elseif client?? && client.baseUrl?has_content>
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
