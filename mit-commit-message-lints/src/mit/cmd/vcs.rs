use crate::external::Vcs;

use crate::mit::cmd::errors::Error;

#[allow(clippy::maybe_infinite_iter)]
pub(crate) fn get_vcs_coauthors_config<'a>(
    config: &'a dyn Vcs,
    key: &'a str,
) -> Result<Vec<Option<&'a str>>, Error> {
    (0..)
        .take_while(|index| has_vcs_coauthor(config, *index))
        .map(|index| get_vcs_coauthor_config(config, key, index))
        .fold(Ok(Vec::<Option<&'a str>>::new()), |acc, item| {
            match (acc, item) {
                (Err(error), _) | (Ok(_), Err(error)) => Err(error),
                (Ok(list), Ok(item)) => Ok(vec![list, vec![item]].concat()),
            }
        })
}

pub(crate) fn has_vcs_coauthor(config: &dyn Vcs, index: i32) -> bool {
    let email = get_vcs_coauthor_config(config, "email", index);
    let name = get_vcs_coauthor_config(config, "name", index);

    if let (Ok(Some(_)), Ok(Some(_))) = (name, email) {
        true
    } else {
        false
    }
}

pub(crate) fn get_vcs_coauthor_config<'a>(
    config: &'a dyn Vcs,
    key: &str,
    index: i32,
) -> Result<Option<&'a str>, Error> {
    config
        .get_str(&format!("mit.author.coauthors.{}.{}", index, key))
        .map_err(Error::from)
}
