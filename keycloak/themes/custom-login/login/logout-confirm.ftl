<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=true; section>
    <#if section = "header">
        <#-- No header title -->
    <#elseif section = "form">
        <div id="kc-form">
            <div id="kc-form-wrapper">
                <div class="login-card">
                    <div class="login-header">
                        <h1>Sign Out</h1>
                        <p>Are you sure you want to sign out?</p>
                    </div>
                    
                    <form class="form-actions" action="${url.logoutConfirmAction}" method="POST">
                        <#if logoutConfirm?? && logoutConfirm.code??>
                            <input type="hidden" name="session_code" value="${logoutConfirm.code}">
                        </#if>
                        
                        <div class="form-buttons logout-buttons">
                            <input tabindex="4" class="login-btn logout-btn" name="confirmLogout" id="kc-logout" type="submit" value="Sign Out"/>
                            <#if logoutConfirm?? && logoutConfirm.skipLink?? && logoutConfirm.skipLink?is_string>
                                <a href="${logoutConfirm.skipLink}" class="cancel-btn">Cancel</a>
                            <#else>
                                <a href="${url.loginUrl}" class="cancel-btn">Cancel</a>
                            </#if>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    </#if>
</@layout.registrationLayout>
