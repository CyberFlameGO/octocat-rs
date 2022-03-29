use crate::model::{organizations::Organization, user::User};

use super::prelude::*;

/// * tags users
/// * get `/user/followers`
/// * docs <https://docs.github.com/rest/reference/users#list-followers-of-the-authenticated-user>
///
/// List followers of the authenticated user
/// Lists the people following the authenticated user.
pub async fn get_followers<T>(client: &T, params: Option<&Pagination>) -> Result<Vec<User>, GithubRestError>
where
    T: Requester,
{
    client
        .req::<Pagination, String, Vec<User>>(EndPoints::GetUserFollowers(), params, None)
        .await
}

/// * tags users
/// * get `/user/following`
/// * docs <https://docs.github.com/rest/reference/users#list-the-people-the-authenticated-user-follows>
///
/// List the people the authenticated user follows
/// Lists the people who the authenticated user follows.
pub async fn get_following<T>(client: &T, params: Option<&Pagination>) -> Result<Vec<User>, GithubRestError>
where
    T: Requester,
{
    client
        .req::<Pagination, String, Vec<User>>(EndPoints::GetUserFollowing(), params, None)
        .await
}

user_and_pagination_methods!(
    /// * tags users
    /// * get `/users/{username}/keys`
    /// * docs <https://docs.github.com/rest/reference/users#list-public-keys-for-a-user>
    ///
    /// List public keys for a user
    /// Lists the _verified_ public SSH keys for a user. This is accessible by
    /// anyone.
    get_user_keys,
    EndPoints::GetUsersusernameKeys,
    Vec<Key>,
    /// * tags orgs
    /// * get `/users/{username}/orgs`
    /// * docs <https://docs.github.com/rest/reference/orgs#list-organizations-for-a-user>
    ///
    /// List organizations for a user
    /// List [public organization memberships](https://docs.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
    ///
    /// This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/reference/orgs#list-organizations-for-the-authenticated-user) API instead.
    get_user_organizations,
    EndPoints::GetUsersusernameOrgs,
    Vec<Organization>,
    /// * tags users
    /// * get `/users/{username}/following`
    /// * docs <https://docs.github.com/rest/reference/users#list-the-people-a-user-follows>
    ///
    /// List the people a user follows
    /// Lists the people who the specified user follows.
    get_user_following,
    EndPoints::GetUsersusernameFollowing,
    Vec<User>,
    /// * tags users
    /// * get `/users/{username}/followers`
    /// * docs <https://docs.github.com/rest/reference/users#list-followers-of-a-user>
    ///
    /// List followers of a user
    /// Lists the people following the specified user.
    get_user_followers,
    EndPoints::GetUsersusernameFollowers,
    Vec<User>
);

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Key {
    pub id: usize,
    pub key: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Pagination {
    /// Results per page (max 100)
    /// Default: 30
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,
    /// Page number of the results to fetch.
    /// Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequester;

    use super::*;

    #[tokio::test]
    async fn test_get_followers() {
        // Note: this requires auth
        let client = DefaultRequester::new_none();
        let res = get_followers(&client, None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_get_following() {
        // Note: this requires auth
        let client = DefaultRequester::new_none();
        let res = get_followers(&client, None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_get_user_following() {
        let client = DefaultRequester::new_none();
        let res = get_user_following(&client, "proudmuslim-dev", None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_get_user_followers() {
        let client = DefaultRequester::new_none();
        let res = get_user_followers(&client, "proudmuslim-dev", None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_get_user_organizations() {
        let client = DefaultRequester::new_none();
        let res = get_user_organizations(&client, "proudmuslim-dev", None).await.unwrap();
        dbg!(res);
    }
    #[tokio::test]
    async fn test_get_user_keys() {
        let client = DefaultRequester::new_none();
        let res = get_user_keys(&client, "proudmuslim-dev", None).await.unwrap();
        dbg!(res);
    }
}
